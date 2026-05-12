/// src/ecc/field.rs
use num_bigint::BigUint;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    num: BigUint,
    prime: BigUint,
}

impl FieldElement {
    pub fn new(num: BigUint, prime: BigUint) -> Self {
        if num >= prime {
            panic!("num not in range 0 to {}", prime - BigUint::from(1u32));
        }
        FieldElement { num, prime }
    }

    pub fn zero(prime: BigUint) -> Self {
        Self::new(BigUint::from(0u32), prime)
    }
    pub fn one(prime: BigUint) -> Self {
        Self::new(BigUint::from(1u32), prime)
    }

    // Convenience constructor for u64 values
    pub fn from_u64(num: u64, prime: u64) -> Self {
        Self::new(BigUint::from(num), BigUint::from(prime))
    }

    // Convenience constructor for hex strings
    pub fn from_hex(num_hex: &str, prime_hex: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let num =
            BigUint::parse_bytes(num_hex.as_bytes(), 16).ok_or("Invalid hex string for num")?;
        let prime =
            BigUint::parse_bytes(prime_hex.as_bytes(), 16).ok_or("Invalid hex string for prime")?;
        Ok(Self::new(num, prime))
    }

    // Convenience constructor for bytes (big-endian)
    pub fn from_bytes(num_bytes: &[u8], prime_bytes: &[u8]) -> Self {
        let num = BigUint::from_bytes_be(num_bytes);
        let prime = BigUint::from_bytes_be(prime_bytes);
        Self::new(num, prime)
    }

    // Convenience constructor for creating from num bytes with known prime
    pub fn from_num_bytes(num_bytes: &[u8], prime: BigUint) -> Self {
        let num = BigUint::from_bytes_be(num_bytes);
        Self::new(num, prime)
    }

    // Convert the field element's num to bytes (big-endian)
    pub fn to_bytes(&self) -> Vec<u8> {
        self.num.to_bytes_be()
    }

    // Convert the field element's num to bytes with fixed length (big-endian, zero-padded)
    pub fn to_bytes_fixed(&self, len: usize) -> Vec<u8> {
        let mut bytes = self.num.to_bytes_be();
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

    // Convert both num and prime to bytes
    pub fn to_bytes_with_prime(&self) -> (Vec<u8>, Vec<u8>) {
        (self.num.to_bytes_be(), self.prime.to_bytes_be())
    }

    pub fn inverse(&self) -> Self {
        // Fermat's Little Theorem: a^(p-1) ≡ 1 (mod p), so a^(p-2) is the inverse
        self.pow(&self.prime - BigUint::from(2u32))
    }

    pub fn is_zero(&self) -> bool {
        self.num == BigUint::from(0u32)
    }

    pub fn num(&self) -> &BigUint {
        &self.num
    }

    pub fn prime(&self) -> &BigUint {
        &self.prime
    }

    pub fn sqrt(&self) -> Self {
        let exp = (&self.prime + BigUint::from(1u32)) / BigUint::from(4u32);
        let result = self.num.modpow(&exp, &self.prime);
        Self::new(result, self.prime.clone())
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FieldElement_num_{}_prime_{}",
            self.num.to_str_radix(16),
            self.prime.to_str_radix(16)
        )
    }
}

impl Add for &FieldElement {
    type Output = FieldElement;

    fn add(self, other: Self) -> FieldElement {
        assert_eq!(
            self.prime, other.prime,
            "Cannot add two numbers in different fields"
        );
        let num = (&self.num + &other.num) % &self.prime;
        FieldElement {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl Sub for &FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> FieldElement {
        assert_eq!(
            self.prime, rhs.prime,
            "Cannot subtract two numbers in different fields"
        );
        let rs = if self.num >= rhs.num {
            (&self.num - &rhs.num) % &self.prime
        } else {
            (&self.prime + &self.num - &rhs.num) % &self.prime
        };
        FieldElement {
            num: rs,
            prime: self.prime.clone(),
        }
    }
}

impl Mul for &FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> FieldElement {
        assert_eq!(
            self.prime, rhs.prime,
            "Cannot multiply two numbers in different fields"
        );
        let rs = (self.num() * rhs.num()) % self.prime();
        FieldElement {
            num: rs,
            prime: self.prime().clone(),
        }
    }
}

impl Mul<u32> for &FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: u32) -> FieldElement {
        let rs = (self.num() * BigUint::from(rhs)) % self.prime();
        FieldElement {
            num: rs,
            prime: self.prime().clone(),
        }
    }
}

impl Div for &FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: &FieldElement) -> FieldElement {
        assert!(!rhs.is_zero(), "Zero is an invalid denominator!");

        assert_eq!(
            self.prime, rhs.prime,
            "Cannot divide two numbers in different fields"
        );

        let rs = (self.num() * rhs.inverse().num()) % self.prime();
        FieldElement {
            num: rs,
            prime: self.prime().clone(),
        }
    }
}

pub trait Pow<T> {
    type Output;
    fn pow(self, exp: T) -> Self::Output;
}

impl Pow<&BigUint> for &FieldElement {
    type Output = FieldElement;

    fn pow(self, exp: &BigUint) -> FieldElement {
        let rs = self.num().modpow(exp, self.prime());
        FieldElement {
            num: rs,
            prime: self.prime().clone(),
        }
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
