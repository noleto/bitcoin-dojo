use bitcoin_dojo::ecc::scalar::*;
#[cfg(test)]
use num_bigint::BigUint;

#[test]
fn test_scalar_creation() {
    let n = BigUint::from(17u32);
    let scalar = Scalar::new(BigUint::from(20u32), n.clone());
    assert_eq!(scalar.value, BigUint::from(3u32)); // 20 % 17 = 3
}

#[test]
fn test_scalar_addition() {
    let n = BigUint::from(17u32);
    let a = Scalar::new(BigUint::from(10u32), n.clone());
    let b = Scalar::new(BigUint::from(12u32), n.clone());
    let result = a + b;
    assert_eq!(result.value, BigUint::from(5u32)); // (10 + 12) % 17 = 5
}

#[test]
fn test_scalar_subtraction() {
    let n = BigUint::from(17u32);
    let a = Scalar::new(BigUint::from(5u32), n.clone());
    let b = Scalar::new(BigUint::from(10u32), n.clone());
    let result = a - b;
    assert_eq!(result.value, BigUint::from(12u32)); // (5 - 10) % 17 = 12
}

#[test]
fn test_scalar_multiplication() {
    let n = BigUint::from(17u32);
    let a = Scalar::new(BigUint::from(3u32), n.clone());
    let b = Scalar::new(BigUint::from(5u32), n.clone());
    let result = a * b;
    assert_eq!(result.value, BigUint::from(15u32)); // (3 * 5) % 17 = 15
}

#[test]
fn test_scalar_inverse() {
    let n = BigUint::from(17u32);
    let scalar = Scalar::new(BigUint::from(3u32), n.clone());
    let inverse = scalar.inverse().unwrap();
    let product = &scalar * &inverse;
    assert_eq!(product.value, BigUint::from(1u32));
}

#[test]
fn test_bytes_conversion() {
    let n = BigUint::from(17u32);
    let scalar = Scalar::new(BigUint::from(255u32), n.clone());
    let bytes = scalar.as_bytes();
    let reconstructed = Scalar::from_bytes(&bytes, n);
    assert_eq!(scalar.value, reconstructed.value);
}

// Additional tests for better coverage:

#[test]
fn test_scalar_zero() {
    let n = BigUint::from(17u32);
    let zero = Scalar::zero(n.clone());
    assert_eq!(zero.value, BigUint::from(0u32));

    // Test zero addition
    let a = Scalar::new(BigUint::from(5u32), n.clone());
    let result = &a + &zero;
    assert_eq!(result.value, a.value);
}

#[test]
fn test_scalar_one() {
    let n = BigUint::from(17u32);
    let one = Scalar::one(n.clone());
    assert_eq!(one.value, BigUint::from(1u32));

    // Test one multiplication
    let a = Scalar::new(BigUint::from(5u32), n.clone());
    let result = &a * &one;
    assert_eq!(result.value, a.value);
}

#[test]
fn test_scalar_inverse_edge_cases() {
    let n = BigUint::from(17u32);

    // Test inverse of 1
    let one = Scalar::new(BigUint::from(1u32), n.clone());
    let inverse = one.inverse().unwrap();
    assert_eq!(inverse.value, BigUint::from(1u32));

    // Test inverse of zero should fail
    let zero = Scalar::new(BigUint::from(0u32), n.clone());
    assert!(zero.inverse().is_none());

    // Test inverse of n-1 (which is -1 mod n)
    let minus_one = Scalar::new(BigUint::from(16u32), n.clone());
    let inverse = minus_one.inverse().unwrap();
    assert_eq!(inverse.value, BigUint::from(16u32)); // -1 is its own inverse
}

#[test]
fn test_scalar_inverse_composite_modulus() {
    // Test with composite modulus where not all elements have inverses
    let n = BigUint::from(15u32); // 15 = 3 * 5

    // 3 should not have an inverse mod 15
    let three = Scalar::new(BigUint::from(3u32), n.clone());
    assert!(three.inverse().is_none());

    // 5 should not have an inverse mod 15
    let five = Scalar::new(BigUint::from(5u32), n.clone());
    assert!(five.inverse().is_none());

    // 2 should have an inverse mod 15
    let two = Scalar::new(BigUint::from(2u32), n.clone());
    let inverse = two.inverse().unwrap();
    let product = &two * &inverse;
    assert_eq!(product.value, BigUint::from(1u32));
}

#[test]
fn test_scalar_addition_overflow() {
    let n = BigUint::from(17u32);
    let a = Scalar::new(BigUint::from(16u32), n.clone());
    let b = Scalar::new(BigUint::from(16u32), n.clone());
    let result = a + b;
    assert_eq!(result.value, BigUint::from(15u32)); // (16 + 16) % 17 = 15
}

#[test]
fn test_scalar_subtraction_underflow() {
    let n = BigUint::from(17u32);
    let a = Scalar::new(BigUint::from(3u32), n.clone());
    let b = Scalar::new(BigUint::from(10u32), n.clone());
    let result = a - b;
    assert_eq!(result.value, BigUint::from(10u32)); // (3 - 10) % 17 = 10
}

#[test]
fn test_scalar_multiplication_large() {
    let n = BigUint::from(17u32);
    let a = Scalar::new(BigUint::from(16u32), n.clone());
    let b = Scalar::new(BigUint::from(16u32), n.clone());
    let result = a * b;
    assert_eq!(result.value, BigUint::from(1u32)); // (16 * 16) % 17 = 256 % 17 = 1
}

#[test]
fn test_scalar_borrowed_operations() {
    let n = BigUint::from(17u32);
    let a = Scalar::new(BigUint::from(10u32), n.clone());
    let b = Scalar::new(BigUint::from(7u32), n.clone());

    // Test borrowed addition
    let result = &a + &b;
    assert_eq!(result.value, BigUint::from(0u32)); // (10 + 7) % 17 = 0

    // Test borrowed subtraction
    let result = &a - &b;
    assert_eq!(result.value, BigUint::from(3u32)); // (10 - 7) % 17 = 3

    // Test borrowed multiplication
    let result = &a * &b;
    assert_eq!(result.value, BigUint::from(2u32)); // (10 * 7) % 17 = 2
}

#[test]
fn test_scalar_large_numbers() {
    let n = BigUint::parse_bytes(
        b"115792089237316195423570985008687907852837564279074904382605163141518161494337",
        10,
    )
    .unwrap(); // secp256k1 order
    let large_val = BigUint::parse_bytes(
        b"123456789012345678901234567890123456789012345678901234567890",
        10,
    )
    .unwrap();

    let scalar = Scalar::new(large_val, n.clone());
    let bytes = scalar.as_bytes();
    let reconstructed = Scalar::from_bytes(&bytes, n);
    assert_eq!(scalar.value, reconstructed.value);
}

#[test]
fn test_scalar_bytes_edge_cases() {
    let n = BigUint::from(256u32);

    // Test with zero
    let zero = Scalar::zero(n.clone());
    let bytes = zero.as_bytes();
    let reconstructed = Scalar::from_bytes(&bytes, n.clone());
    assert_eq!(zero.value, reconstructed.value);

    // Test with max single byte value
    let max_byte = Scalar::new(BigUint::from(255u32), n.clone());
    let bytes = max_byte.as_bytes();
    let reconstructed = Scalar::from_bytes(&bytes, n);
    assert_eq!(max_byte.value, reconstructed.value);
}

#[test]
fn test_scalar_equality() {
    let n = BigUint::from(17u32);
    let a = Scalar::new(BigUint::from(5u32), n.clone());
    let b = Scalar::new(BigUint::from(5u32), n.clone());
    let c = Scalar::new(BigUint::from(22u32), n.clone()); // 22 % 17 = 5

    assert_eq!(a, b);
    assert_eq!(a, c); // Should be equal due to modular reduction
}

#[test]
fn test_scalar_getters() {
    let n = BigUint::from(17u32);
    let scalar = Scalar::new(BigUint::from(20u32), n.clone());

    assert_eq!(*scalar.value(), BigUint::from(3u32));
    assert_eq!(*scalar.modulus(), n);
}

#[test]
#[should_panic(expected = "Cannot add scalars with different moduli")]
fn test_scalar_different_moduli_addition() {
    let a = Scalar::new(BigUint::from(5u32), BigUint::from(17u32));
    let b = Scalar::new(BigUint::from(3u32), BigUint::from(19u32));
    let _ = a + b;
}

#[test]
#[should_panic(expected = "Cannot subtract scalars with different moduli")]
fn test_scalar_different_moduli_subtraction() {
    let a = Scalar::new(BigUint::from(5u32), BigUint::from(17u32));
    let b = Scalar::new(BigUint::from(3u32), BigUint::from(19u32));
    let _ = a - b;
}

#[test]
#[should_panic(expected = "Cannot multiply scalars with different moduli")]
fn test_scalar_different_moduli_multiplication() {
    let a = Scalar::new(BigUint::from(5u32), BigUint::from(17u32));
    let b = Scalar::new(BigUint::from(3u32), BigUint::from(19u32));
    let _ = a * b;
}

#[test]
fn test_scalar_associativity() {
    let n = BigUint::from(17u32);
    let a = Scalar::new(BigUint::from(3u32), n.clone());
    let b = Scalar::new(BigUint::from(5u32), n.clone());
    let c = Scalar::new(BigUint::from(7u32), n.clone());

    // Test addition associativity: (a + b) + c = a + (b + c)
    let ab = &a + &b;
    let left = &ab + &c;
    let bc = &b + &c;
    let right = &a + &bc;
    assert_eq!(left, right);

    // Test multiplication associativity: (a * b) * c = a * (b * c)
    let ab = &a * &b;
    let left = &ab * &c;
    let bc = &b * &c;
    let right = &a * &bc;
    assert_eq!(left, right);
}

#[test]
fn test_scalar_commutativity() {
    let n = BigUint::from(17u32);
    let a = Scalar::new(BigUint::from(3u32), n.clone());
    let b = Scalar::new(BigUint::from(5u32), n.clone());

    // Test addition commutativity: a + b = b + a
    assert_eq!(&a + &b, &b + &a);

    // Test multiplication commutativity: a * b = b * a
    assert_eq!(&a * &b, &b * &a);
}
