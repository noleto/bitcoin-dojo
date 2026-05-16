use num_bigint::BigUint;
use rand::Rng;
use sha2::{Digest, Sha256};

pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn secure_random_bytes(len: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; len];
    rand::rng().fill_bytes(&mut bytes);
    bytes
}

pub fn secure_random_scalar() -> num_bigint::BigUint {
    use super::constants as consts;
    loop {
        let bytes = secure_random_bytes(32);
        let candidate = num_bigint::BigUint::from_bytes_be(&bytes);
        if candidate < *consts::SECP256K1_N && &candidate > consts::bigint_zero() {
            return candidate;
        }
    }
}

// Extended Euclidean Algorithm for modular inverse
// Returns (gcd, x) where x is the modular inverse of a mod m
pub fn extended_gcd_for_inverse(a: &BigUint, m: &BigUint) -> (BigUint, BigUint) {
    if a == &BigUint::from(0u32) {
        return (m.clone(), BigUint::from(0u32));
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
        m - (&old_s % m)
    } else {
        old_s % m
    };

    (old_r, result)
}
