/// src/ecc/ecdsa.rs
use super::constants::SECP256K1_N;
use super::curve::Point;
use super::keys::{PrivateKey, PublicKey};
use super::scalar::Scalar;
use hmac::{Hmac, Mac};
use num_bigint::BigUint;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    pub r: Scalar,
    pub s: Scalar,
}

/// Generate deterministic k value according to RFC 6979
/// This ensures that the same message and private key always produce the same signature
fn deterministic_k(private_key: &PrivateKey, message_hash: &[u8]) -> Scalar {
    // Convert message hash to scalar
    let mut z = Scalar::new(BigUint::from_bytes_be(message_hash));

    // Get the secp256k1 order (n)
    let n = &*SECP256K1_N;

    // Adjust z if it's >= n (reduce modulo n)
    if z.value() >= n {
        z = Scalar::new(z.value() % n);
    }

    // Convert private key and z to 32-byte arrays
    let private_key_bytes = private_key.scalar().value().to_bytes_be();
    let mut private_key_32 = [0u8; 32];
    let start_idx = if private_key_bytes.len() < 32 {
        32 - private_key_bytes.len()
    } else {
        0
    };
    private_key_32[start_idx..]
        .copy_from_slice(&private_key_bytes[private_key_bytes.len().saturating_sub(32)..]);

    let z_bytes = z.value().to_bytes_be();
    let mut z_32 = [0u8; 32];
    let z_start_idx = if z_bytes.len() < 32 {
        32 - z_bytes.len()
    } else {
        0
    };
    z_32[z_start_idx..].copy_from_slice(&z_bytes[z_bytes.len().saturating_sub(32)..]);

    // Step 1: Initialize K and V
    let mut k = vec![0u8; 32];
    let mut v = vec![1u8; 32];

    // Step 2: First HMAC round with 0x00
    let mut data = Vec::new();
    data.extend_from_slice(&v);
    data.push(0x00);
    data.extend_from_slice(&private_key_32);
    data.extend_from_slice(&z_32);

    let mut hmac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
    hmac.update(&data);
    k = hmac.finalize().into_bytes().to_vec();

    // Update V
    let mut hmac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
    hmac.update(&v);
    v = hmac.finalize().into_bytes().to_vec();

    // Step 3: Second HMAC round with 0x01
    let mut data = Vec::new();
    data.extend_from_slice(&v);
    data.push(0x01);
    data.extend_from_slice(&private_key_32);
    data.extend_from_slice(&z_32);

    let mut hmac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
    hmac.update(&data);
    k = hmac.finalize().into_bytes().to_vec();

    // Update V
    let mut hmac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
    hmac.update(&v);
    v = hmac.finalize().into_bytes().to_vec();

    // Step 4: Generate candidate k values until we find a valid one
    loop {
        // Generate V
        let mut hmac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
        hmac.update(&v);
        v = hmac.finalize().into_bytes().to_vec();

        // Convert V to BigUint
        let candidate = BigUint::from_bytes_be(&v);

        // Check if candidate is in valid range [1, n-1]
        if candidate >= BigUint::from(1u32) && candidate < *n {
            return Scalar::new(candidate);
        }

        // Update K and V for next iteration
        let mut data = Vec::new();
        data.extend_from_slice(&v);
        data.push(0x00);

        let mut hmac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
        hmac.update(&data);
        k = hmac.finalize().into_bytes().to_vec();

        let mut hmac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
        hmac.update(&v);
        v = hmac.finalize().into_bytes().to_vec();
    }
}

/// ECDSA signature using deterministic k generation (RFC 6979)
/// This ensures that the same message and private key always produce the same signature
/// NEVER user this signature in prod for real signing tools
/// YOU RISK TO LEAK YOUR PRIVATE KEY => https://learnmeabitcoin.com/technical/cryptography/elliptic-curve/ecdsa/#private-key-recovery
pub fn sign(private_key: &PrivateKey, message_hash: &[u8]) -> Signature {
    let k = deterministic_k(private_key, message_hash);
    let rnd_point = Point::generator() * &k;

    let k_modular_inverse = k
        .inverse()
        .expect("Random point has no inverse, cannot produce ECDSA signature");
    let fe_k = rnd_point
        .x()
        .as_ref()
        .expect("Random point is at infinity, cannot produce ECDSA signatur");
    let r_x = Scalar::new(fe_k.value().clone());
    let z = Scalar::new(BigUint::from_bytes_be(message_hash));
    let d = private_key.scalar();
    let s = k_modular_inverse * (z + d * &r_x);

    Signature { r: r_x, s }
}

pub fn verify(public_key: &PublicKey, message_hash: &[u8], signature: &Signature) -> bool {
    let z = Scalar::new(BigUint::from_bytes_be(message_hash));
    signature.s.inverse().as_ref().map_or(false, |s_inverse| {
        let point1 = Point::generator() * (s_inverse * &z);
        let point2 = public_key.point() * (s_inverse * &signature.r);
        let point3 = point1 + point2;
        point3
            .x()
            .as_ref()
            .map_or(false, |x| x.value() == signature.r.value())
    })
}

impl Signature {
    /// Encode the signature in Distinguished Encoding Rules (DER) format
    ///
    /// DER format for ECDSA signatures:
    /// SEQUENCE {
    ///   r INTEGER,
    ///   s INTEGER
    /// }
    pub fn to_der(&self) -> Vec<u8> {
        let mut der = Vec::new();
        der.push(0x30);

        let r_der = self.encode_integer(&self.r);
        let s_der = self.encode_integer(&self.s);

        der.push(self.encode_length(s_der.len() + r_der.len()));

        der.extend(r_der);
        der.extend(s_der);

        der
    }

    /// Parse a DER-encoded signature
    ///
    /// Returns None if the DER encoding is invalid
    pub fn from_der(der_bytes: &[u8]) -> Option<Self> {
        //Check if matches DER expected lentghts only
        if !(70..=72).contains(&der_bytes.len()) {
            return None;
        }

        if der_bytes[0] != 0x30 {
            return None;
        }

        let size_r = der_bytes[3] as usize;
        let (r, der_r_size) = Self::decode_integer(&der_bytes[2..(2 + 2 + size_r)])?;
        let (s, _) = Self::decode_integer(&der_bytes[(2 + der_r_size)..])?;
        Some(Signature { r, s })
    }

    /// (Optional) helper methods

    /// Encode a scalar as a DER INTEGER
    fn encode_integer(&self, scalar: &Scalar) -> Vec<u8> {
        let mut der_int_bytes = Vec::with_capacity(35);
        der_int_bytes.push(0x02);

        let scalar_bytes = scalar.as_bytes();
        let needs_padding = scalar_bytes[0] >= 0x80;
        der_int_bytes.push(self.encode_length(scalar_bytes.len() + needs_padding as usize));
        if needs_padding {
            der_int_bytes.push(0x00);
        }
        der_int_bytes.extend(scalar_bytes);

        der_int_bytes
    }

    /// Encode length in DER format
    fn encode_length(&self, length: usize) -> u8 {
        assert!(length <= 255, "Cannot encode length greather than 255");
        let as_bytes = length.to_be_bytes();
        as_bytes
            .last()
            .expect("Cannot encode length as byte representation is empty! ")
            .clone()
    }

    /// Decode DER-encoded INTEGER
    /// 0x02 + length + integer_bytes
    fn decode_integer(bytes: &[u8]) -> Option<(Scalar, usize)> {
        if !(34..=35).contains(&bytes.len()) {
            return None;
        }
        if bytes[0] != 0x02 {
            return None;
        }

        if !matches!(bytes[1], 0x20..=0x21) {
            return None;
        }

        Some((
            Scalar::new(BigUint::from_bytes_be(&bytes[2..])),
            bytes.len(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;
    use sha2::{Digest, Sha256};

    pub fn sha256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    fn biguint_from_hex(hex_str: &str) -> BigUint {
        BigUint::from_bytes_be(&hex::decode(hex_str).unwrap())
    }

    #[test]
    fn test_verify() {
        //vector test from https://learnmeabitcoin.com/technical/cryptography/elliptic-curve/ecdsa/#verify
        let message_hash = sha256(b"ECDSA is the most fun I have ever experienced");
        let priv_k = PrivateKey::new_with_seed(biguint_from_hex(
            "f94a840f1e1a901843a75dd07ffcc5c84478dc4f987797474c9393ac53ab55e6",
        ));
        let sig = Signature {
            r: Scalar::new(biguint_from_hex(
                "f01d6b9018ab421dd410404cb869072065522bf85734008f105cf385a023a80f",
            )),
            s: Scalar::new(biguint_from_hex(
                "a3243a18521b20dc80a8798a1a36463ffe8279574127da214d39e6b34134305b",
            )),
        };
        assert!(verify(&priv_k.public_key(), &message_hash, &sig))
    }

    #[test]
    fn test_der_encoding() {
        let sig = Signature {
            r: Scalar::new(
                BigUint::parse_bytes(
                    b"4b3c7f0bf30231753bf6ba2d55d7ffe5366d42a0132c96c0be662a84bb089bac",
                    16,
                )
                .expect("Cannot parse into BigUint"),
            ),
            s: Scalar::new(
                BigUint::parse_bytes(
                    b"309a96c2301de90875910ec90b1927dd2db76209b0fb9f943f44dfe4c52597f7",
                    16,
                )
                .expect("Cannot parse into BigUint"),
            ),
        };

        assert_eq!(
            hex::encode(sig.to_der()),
            "304402204b3c7f0bf30231753bf6ba2d55d7ffe5366d42a0132c96c0be662a84bb089bac0220309a96c2301de90875910ec90b1927dd2db76209b0fb9f943f44dfe4c52597f7"
        )
    }

    #[test]
    fn test_der_decoding() {
        let sig = Signature {
            r: Scalar::new(
                BigUint::parse_bytes(
                    b"4b3c7f0bf30231753bf6ba2d55d7ffe5366d42a0132c96c0be662a84bb089bac",
                    16,
                )
                .expect("Cannot parse into BigUint"),
            ),
            s: Scalar::new(
                BigUint::parse_bytes(
                    b"309a96c2301de90875910ec90b1927dd2db76209b0fb9f943f44dfe4c52597f7",
                    16,
                )
                .expect("Cannot parse into BigUint"),
            ),
        };

        let sig_recreated = Signature::from_der(&hex::decode(
            "304402204b3c7f0bf30231753bf6ba2d55d7ffe5366d42a0132c96c0be662a84bb089bac0220309a96c2301de90875910ec90b1927dd2db76209b0fb9f943f44dfe4c52597f7",
        ).expect("hex failed to decode"));

        assert_eq!(sig, sig_recreated.expect("Cannot decode"));
    }
}
