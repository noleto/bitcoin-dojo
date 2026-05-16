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
}
