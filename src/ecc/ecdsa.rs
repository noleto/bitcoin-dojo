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
    fn feature() {
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
}
