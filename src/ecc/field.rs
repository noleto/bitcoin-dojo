use crate::ecc::constants as consts;
use crate::ecc::util::extended_gcd_for_inverse;
use num_bigint::BigUint;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    value: BigUint,
}

impl FieldElement {
    pub fn new(value: BigUint) -> Self {
        FieldElement {
            value: value % &*consts::SECP256K1_P,
        }
    }

    pub fn zero() -> FieldElement {
        FieldElement::new(consts::bigint_zero().clone())
    }

    pub fn one() -> FieldElement {
        FieldElement::new(consts::bigint_one().clone())
    }

    // Convenience constructor for u64 values
    pub fn from_u64(num: u64) -> Self {
        Self::new(BigUint::from(num))
    }

    // Convenience constructor for hex strings
    pub fn from_hex(num_hex: &str) -> Result<FieldElement, Box<dyn std::error::Error>> {
        let num =
            BigUint::parse_bytes(num_hex.as_bytes(), 16).ok_or("Invalid hex string for num")?;
        Ok(Self::new(num))
    }

    // Convenience constructor for bytes (big-endian)
    pub fn from_bytes(num_bytes: &[u8]) -> FieldElement {
        let num = BigUint::from_bytes_be(num_bytes);
        Self::new(num)
    }

    // Convert the field element's num to bytes (big-endian)
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_bytes_be()
    }

    // Convert the field element's num to bytes with fixed length (big-endian, zero-padded)
    pub fn to_bytes_fixed(&self, len: usize) -> Vec<u8> {
        let mut bytes = self.value.to_bytes_be();
        match bytes.len().cmp(&len) {
            Ordering::Less => {
                let mut padded = vec![0u8; len - bytes.len()];
                padded.extend(bytes);
                padded
            }
            Ordering::Greater => {
                // Take the least significant bytes if the number is too large
                bytes.split_off(bytes.len() - len)
            }
            Ordering::Equal => bytes,
        }
    }

    pub fn inverse(&self) -> Option<Self> {
        if self.value == BigUint::from(0u32) {
            return None;
        }

        // Extended Euclidean Algorithm for modular inverse
        let (gcd, x) = extended_gcd_for_inverse(self.value(), self.prime());

        if gcd != BigUint::from(1u32) {
            return None; // No inverse exists
        }

        Some(Self::new(x))
    }

    pub fn is_zero(&self) -> bool {
        self.value == BigUint::from(0u32)
    }

    pub fn value(&self) -> &BigUint {
        &self.value
    }

    pub fn prime(&self) -> &BigUint {
        &*consts::SECP256K1_P
    }

    pub fn sqrt(&self) -> Self {
        //x((p+1)/4)
        let exp = (self.prime() + consts::bigint_one()) / BigUint::from(4u32);
        let result = self.value().modpow(&exp, self.prime());
        Self::new(result)
    }

    pub fn secp256k1_fe_a() -> &'static FieldElement {
        static SECP256K1_FE_A: OnceLock<FieldElement> = OnceLock::new();
        SECP256K1_FE_A.get_or_init(|| FieldElement::new(consts::SECP256K1_A.clone()))
    }

    pub fn secp256k1_fe_b() -> &'static FieldElement {
        static SECP256K1_FE_B: OnceLock<FieldElement> = OnceLock::new();
        SECP256K1_FE_B.get_or_init(|| FieldElement::new(consts::SECP256K1_B.clone()))
    }

    pub fn secp256k1_fe_gx() -> &'static FieldElement {
        static SECP256K1_FE_GX: OnceLock<FieldElement> = OnceLock::new();
        SECP256K1_FE_GX.get_or_init(|| FieldElement::new(consts::SECP256K1_GX.clone()))
    }

    pub fn secp256k1_fe_gy() -> &'static FieldElement {
        static SECP256K1_FE_GY: OnceLock<FieldElement> = OnceLock::new();
        SECP256K1_FE_GY.get_or_init(|| FieldElement::new(consts::SECP256K1_GY.clone()))
    }
}

impl PartialEq<FieldElement> for &FieldElement {
    fn eq(&self, other: &FieldElement) -> bool {
        self.eq(&other)
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}", self.value().to_str_radix(16),)
    }
}
impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, other: FieldElement) -> FieldElement {
        &self + &other
    }
}

impl Add<&FieldElement> for FieldElement {
    type Output = FieldElement;

    fn add(self, other: &FieldElement) -> FieldElement {
        &self + other
    }
}

impl Add<FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn add(self, other: FieldElement) -> FieldElement {
        self + &other
    }
}

impl Add for &FieldElement {
    type Output = FieldElement;

    fn add(self, other: Self) -> FieldElement {
        assert_eq!(
            self.prime(),
            other.prime(),
            "Cannot add two numbers in different fields"
        );
        let value = (&self.value + &other.value) % self.prime();
        FieldElement { value }
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, other: FieldElement) -> FieldElement {
        &self - &other
    }
}

impl Sub<&FieldElement> for FieldElement {
    type Output = FieldElement;

    fn sub(self, other: &FieldElement) -> FieldElement {
        &self - other
    }
}

impl Sub<FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn sub(self, other: FieldElement) -> FieldElement {
        self - &other
    }
}

impl Sub for &FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> FieldElement {
        assert_eq!(
            self.prime(),
            rhs.prime(),
            "Cannot subtract two numbers in different fields"
        );
        let value = if self.value >= rhs.value {
            (self.value() - rhs.value()) % self.prime()
        } else {
            (self.prime() + self.value() - rhs.value()) % self.prime()
        };
        FieldElement { value }
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        &self * &other
    }
}

impl Mul<&FieldElement> for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: &FieldElement) -> FieldElement {
        &self * other
    }
}

impl Mul<FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        self * &other
    }
}

impl Mul for &FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> FieldElement {
        assert_eq!(
            self.prime(),
            rhs.prime(),
            "Cannot multiply two numbers in different fields"
        );
        let value = (self.value() * rhs.value()) % self.prime();
        FieldElement { value }
    }
}

impl Mul<u32> for &FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: u32) -> FieldElement {
        let value = (self.value() * BigUint::from(rhs)) % self.prime();
        FieldElement { value }
    }
}

impl Mul<u32> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: u32) -> FieldElement {
        &self * rhs
    }
}

impl Div for &FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: &FieldElement) -> FieldElement {
        assert!(!rhs.is_zero(), "Division by zero");

        assert_eq!(
            self.prime(),
            rhs.prime(),
            "Cannot divide two numbers in different fields"
        );

        match rhs.inverse() {
            Some(ref rhs_inverse) => {
                let value = (self.value() * rhs_inverse.value()) % self.prime();
                FieldElement { value }
            }
            None => panic!("Cannot divide by a number that has no inverse!"),
        }
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: FieldElement) -> FieldElement {
        &self / &rhs
    }
}

pub trait Pow<T> {
    type Output;
    fn pow(self, exp: T) -> Self::Output;
}

impl Pow<&BigUint> for &FieldElement {
    type Output = FieldElement;

    fn pow(self, exp: &BigUint) -> FieldElement {
        let value = self.value().modpow(exp, self.prime());
        FieldElement { value }
    }
}

impl Pow<BigUint> for &FieldElement {
    type Output = FieldElement;

    fn pow(self, exp: BigUint) -> FieldElement {
        self.pow(&exp)
    }
}

impl Pow<u32> for &FieldElement {
    type Output = FieldElement;

    fn pow(self, exp: u32) -> FieldElement {
        self.pow(BigUint::from(exp))
    }
}
