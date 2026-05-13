/// src/ecc/scalar.rs
use num_bigint::BigUint;
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Scalar {
    pub value: BigUint,
    pub n: BigUint,
}

impl Scalar {
    pub fn new(value: BigUint, n: BigUint) -> Self {
        Self {
            value: value % n.clone(),
            n,
        }
    }

    pub fn inverse(&self) -> Option<Scalar> {
        let (gcd, inverse) = extended_gcd_for_inverse(self.value().clone(), self.modulus().clone());
        (gcd == BigUint::from(1u32)).then(|| Scalar::new(inverse, self.modulus().clone()))
    }

    pub fn as_bytes(&self) -> [u8; 32] {
        let bytes = self.value.to_bytes_be();
        let mut result = [0u8; 32];
        let start = 32 - bytes.len();
        result[start..].copy_from_slice(&bytes);
        result
    }

    pub fn from_bytes(bytes: &[u8; 32], n: BigUint) -> Self {
        let value = BigUint::from_bytes_be(bytes);
        Self::new(value, n)
    }

    pub fn zero(n: BigUint) -> Self {
        Self::new(BigUint::from(0u32), n)
    }

    pub fn one(n: BigUint) -> Self {
        Self::new(BigUint::from(1u32), n)
    }

    pub fn value(&self) -> &BigUint {
        &self.value
    }

    pub fn modulus(&self) -> &BigUint {
        &self.n
    }
}

// Extended Euclidean Algorithm for modular inverse
// Returns (gcd, x) where x is the modular inverse of a mod m
fn extended_gcd_for_inverse(a: BigUint, m: BigUint) -> (BigUint, BigUint) {
    if a == BigUint::from(0u32) {
        return (m, BigUint::from(0u32));
    }

    let mut old_r = a.clone();
    let mut r = m.clone();
    let mut old_s = BigUint::from(1u32);
    let mut s = BigUint::from(0u32);
    let mut old_s_neg = false;
    let mut s_neg = false;

    while r != BigUint::from(0u32) {
        let quotient = &old_r / &r;

        // Update r
        let temp_r = r.clone();
        r = &old_r - &quotient * &r;
        old_r = temp_r;

        // Update s (handling signs)
        let temp_s = s.clone();
        let temp_s_neg = s_neg;

        let product = &quotient * &s;
        if old_s_neg == s_neg {
            if old_s >= product {
                s = &old_s - &product;
                s_neg = old_s_neg;
            } else {
                s = &product - &old_s;
                s_neg = !old_s_neg;
            }
        } else {
            s = &old_s + &product;
            s_neg = old_s_neg;
        }

        old_s = temp_s;
        old_s_neg = temp_s_neg;
    }

    // If old_s is negative, convert to positive equivalent
    let result = if old_s_neg {
        &m - (&old_s % &m)
    } else {
        old_s % &m
    };

    (old_r, result)
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Scalar_value_{}_n_{}",
            self.value.to_str_radix(16),
            self.n.to_str_radix(16)
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
        assert_eq!(self.n, other.n, "Cannot add scalars with different moduli");
        let result = (&self.value + &other.value) % &self.n;
        Scalar::new(result, self.n.clone())
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
            self.n, rhs.n,
            "Cannot subtract scalars with different moduli"
        );
        let rs = if self.value() >= rhs.value() {
            (self.value() - rhs.value()) % self.modulus()
        } else {
            (self.modulus() + self.value() - rhs.value()) % self.modulus()
        };
        Scalar::new(rs, self.modulus().clone())
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
            self.n, rhs.n,
            "Cannot multiply scalars with different moduli"
        );
        let rs = self.value() * rhs.value() % self.modulus();
        Scalar::new(rs, self.modulus().clone())
    }
}
