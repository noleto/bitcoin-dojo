use num_bigint::BigUint;

use crate::ecc::field::FieldElement;
use crate::hash160;
use crate::utils::base58::encode_base58_check;

use super::curve::{Parity, Point};
use super::scalar::Scalar;
use crate::utils::address_types::{AddressType, Network};

#[derive(Debug, Clone)]
pub struct PrivateKey {
    scalar: Scalar,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PublicKey {
    point: Point,
}

impl PrivateKey {
    pub fn new() -> Self {
        Self::from_scalar(Scalar::random())
    }

    pub fn new_with_seed(seed: BigUint) -> Self {
        Self::from_scalar(Scalar::new(seed))
    }

    pub fn from_scalar(scalar: Scalar) -> Self {
        Self { scalar }
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey {
            point: Point::generator() * self.scalar(),
        }
    }

    pub fn scalar(&self) -> &Scalar {
        &self.scalar
    }
}

impl Default for PrivateKey {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicKey {
    pub fn point(&self) -> &Point {
        &self.point
    }

    /// Serialize the public key in SEC format
    /// Returns a 33-byte array for compressed format or 65-byte Vec for uncompressed format
    /// Compressed format: [0x02/0x03, x_coordinate (32 bytes)]
    /// Uncompressed format: [0x04, x_coordinate (32 bytes), y_coordinate (32 bytes)]
    pub fn to_sec(&self, compressed: bool) -> Vec<u8> {
        if compressed {
            self.sec_compressed()
        } else {
            self.sec_uncompressed()
        }
    }

    /// Serialize the public key in compressed SEC format
    /// Returns a 33-byte Vec: [0x02/0x03, x_coordinate (32 bytes)]
    /// 0x02 if y is even, 0x03 if y is odd
    fn sec_compressed(&self) -> Vec<u8> {
        let (Some(x), Some(y)) = (self.point().x().as_ref(), self.point().y().as_ref()) else {
            return Vec::new();
        };

        let mut sec_bytes = Vec::with_capacity(33);
        //push magic byte
        sec_bytes.push(if y.value().bit(0) { 0x03 } else { 0x02 });
        sec_bytes.extend(x.to_bytes_fixed(32));
        sec_bytes
    }

    /// Serialize the public key in uncompressed SEC format
    /// Returns a 65-byte Vec: [0x04, x_coordinate (32 bytes), y_coordinate (32 bytes)]
    fn sec_uncompressed(&self) -> Vec<u8> {
        let (Some(x), Some(y)) = (self.point().x().as_ref(), self.point().y().as_ref()) else {
            return Vec::new();
        };

        let mut sec_bytes: Vec<u8> = Vec::with_capacity(65);
        sec_bytes.push(0x04);
        sec_bytes.extend(x.to_bytes_fixed(32));
        sec_bytes.extend(y.to_bytes_fixed(32));
        sec_bytes
    }

    /// Parse a SEC format public key (compressed or uncompressed)
    /// Compressed format: 33 bytes [0x02/0x03, x_coordinate (32 bytes)]
    /// Uncompressed format: 65 bytes [0x04, x_coordinate (32 bytes), y_coordinate (32 bytes)]
    pub fn parse(sec_bytes: &[u8]) -> Result<Self, &'static str> {
        let bytes_size = sec_bytes.len();
        if bytes_size == 33 {
            Self::parse_compressed(sec_bytes)
        } else if bytes_size == 65 {
            Self::parse_uncompressed(sec_bytes)
        } else {
            Err("Can only parse a compressed (33 bytes) or uncompressed (65 bytes) format.")
        }
    }

    /// Parse an uncompressed SEC format public key
    /// Format: [0x04, x_coordinate (32 bytes), y_coordinate (32 bytes)]
    fn parse_uncompressed(sec_bytes: &[u8]) -> Result<Self, &'static str> {
        if sec_bytes.len() != 65 {
            return Err("Uncompressed SEC format public key should have 65 bytes!");
        }
        if sec_bytes[0] != 0x04 {
            return Err("Uncompressed format must start with 0x04 byte!");
        }
        let x = &sec_bytes[1..33];
        let y = &sec_bytes[33..];
        Ok(PublicKey {
            point: Point::new(
                Some(FieldElement::from_bytes(x)),
                Some(FieldElement::from_bytes(y)),
            ),
        })
    }

    /// Parse a compressed SEC format public key
    /// Format: [0x02/0x03, x_coordinate (32 bytes)]
    fn parse_compressed(sec_bytes: &[u8]) -> Result<Self, &'static str> {
        if sec_bytes.len() != 33 {
            return Err("Compressed SEC format public key should have 33 bytes!");
        }

        let x = Some(FieldElement::from_bytes(&sec_bytes[1..]));
        let parity = match sec_bytes[0] {
            0x02 => Parity::Even,
            0x03 => Parity::Odd,
            _ => return Err("Compressed format must start with 0x02 or 0x03 byte!"),
        };

        Ok(PublicKey {
            point: Point::new_x_only(x, parity),
        })
    }

    /// Generate a Bitcoin address of the specified type
    pub fn address(&self, address_type: AddressType, network: Network) -> String {
        match address_type {
            AddressType::P2PKH => self.p2pkh_address(network),
        }
    }

    /// Convenience method for generating P2PKH addresses
    pub fn p2pkh_address(&self, network: Network) -> String {
        let mut buffer = [0u8; 21];
        buffer[0] = network.p2pkh_version();
        buffer[1..].copy_from_slice(&hash160(&self.sec_compressed()));
        encode_base58_check(&buffer)
    }
}
