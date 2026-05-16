use super::util;
use crate::ecc::constants as consts;
use num_bigint::BigUint;
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Scalar {
    pub value: BigUint,
}

impl Scalar {
    pub fn new(value: BigUint) -> Self {
        Self {
            value: value % &*consts::SECP256K1_N,
        }
    }

    pub fn inverse(&self) -> Option<Scalar> {
        let (ref gcd, inverse) = util::extended_gcd_for_inverse(self.value(), self.modulus());
        (gcd == consts::bigint_one()).then(|| Scalar::new(inverse))
    }

    pub fn as_bytes(&self) -> [u8; 32] {
        let bytes = self.value.to_bytes_be();
        let mut result = [0u8; 32];
        let start = 32 - bytes.len();
        result[start..].copy_from_slice(&bytes);
        result
    }

    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        let value = BigUint::from_bytes_be(bytes);
        Self::new(value)
    }

    pub fn zero() -> Self {
        Self::new(consts::bigint_zero().clone())
    }

    pub fn one() -> Self {
        Self::new(consts::bigint_one().clone())
    }

    pub fn value(&self) -> &BigUint {
        &self.value
    }

    pub fn modulus(&self) -> &BigUint {
        &*consts::SECP256K1_N
    }

    pub fn random() -> Scalar {
        Scalar::new(util::secure_random_scalar())
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Scalar_value_{}_n_{}",
            self.value().to_str_radix(16),
            self.modulus().to_str_radix(16)
        )
    }
}

// Implement arithmetic traits
impl Add for Scalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        (&self).add(&rhs)
    }
}

impl Add for &Scalar {
    type Output = Scalar;

    fn add(self, other: &Scalar) -> Scalar {
        assert_eq!(
            self.modulus(),
            other.modulus(),
            "Cannot add scalars with different moduli"
        );
        let result = (self.value() + other.value()) % self.modulus();
        Scalar::new(result)
    }
}

impl Sub for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Scalar {
        (&self).sub(&rhs)
    }
}

impl Sub for &Scalar {
    type Output = Scalar;

    fn sub(self, rhs: &Scalar) -> Scalar {
        assert_eq!(
            self.modulus(),
            rhs.modulus(),
            "Cannot subtract scalars with different moduli"
        );
        let rs = if self.value() >= rhs.value() {
            (self.value() - rhs.value()) % self.modulus()
        } else {
            (self.modulus() + self.value() - rhs.value()) % self.modulus()
        };
        Scalar::new(rs)
    }
}

impl Mul for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Scalar {
        (&self).mul(&rhs)
    }
}

impl Mul for &Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &Scalar) -> Scalar {
        assert_eq!(
            self.modulus(),
            rhs.modulus(),
            "Cannot multiply scalars with different moduli"
        );
        let rs = self.value() * rhs.value() % self.modulus();
        Scalar::new(rs)
    }
}
