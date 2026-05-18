use num_bigint::BigUint;

use super::curve::Point;
use super::scalar::Scalar;

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
}
