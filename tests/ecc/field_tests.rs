#[cfg(test)]
use bitcoin_dojo::ecc::field::{FieldElement, Pow};
use num_bigint::BigUint;

#[test]
fn test_new_valid() {
    let fe = FieldElement::new(BigUint::from(5u32), BigUint::from(7u32));
    assert_eq!(*fe.num(), BigUint::from(5u32));
    assert_eq!(*fe.prime(), BigUint::from(7u32));
}

#[test]
#[should_panic(expected = "num not in range 0 to 6")]
fn test_new_invalid_num_equal_to_prime() {
    FieldElement::new(BigUint::from(7u32), BigUint::from(7u32));
}

#[test]
#[should_panic(expected = "num not in range 0 to 6")]
fn test_new_invalid_num_greater_than_prime() {
    FieldElement::new(BigUint::from(8u32), BigUint::from(7u32));
}

#[test]
fn test_from_u64() {
    let fe = FieldElement::from_u64(3, 7);
    assert_eq!(*fe.num(), BigUint::from(3u32));
    assert_eq!(*fe.prime(), BigUint::from(7u32));
}

#[test]
fn test_from_hex_valid() {
    let fe = FieldElement::from_hex("a", "b").unwrap();
    assert_eq!(*fe.num(), BigUint::from(10u32));
    assert_eq!(*fe.prime(), BigUint::from(11u32));
}

#[test]
fn test_from_hex_invalid_num() {
    let result = FieldElement::from_hex("xyz", "b");
    assert!(result.is_err());
}

#[test]
fn test_from_hex_invalid_prime() {
    let result = FieldElement::from_hex("a", "xyz");
    assert!(result.is_err());
}

#[test]
fn test_is_zero_true() {
    let fe = FieldElement::new(BigUint::from(0u32), BigUint::from(7u32));
    assert!(fe.is_zero());
}

#[test]
fn test_is_zero_false() {
    let fe = FieldElement::new(BigUint::from(3u32), BigUint::from(7u32));
    assert!(!fe.is_zero());
}

#[test]
fn test_display() {
    let fe = FieldElement::from_u64(10, 17);
    let display_str = format!("{}", fe);
    assert_eq!(display_str, "FieldElement_num_a_prime_11");
}

#[test]
fn test_partial_eq_equal() {
    let fe1 = FieldElement::from_u64(5, 7);
    let fe2 = FieldElement::from_u64(5, 7);
    assert_eq!(&fe1, &fe2);
}

#[test]
fn test_partial_eq_different_num() {
    let fe1 = FieldElement::from_u64(5, 7);
    let fe2 = FieldElement::from_u64(3, 7);
    assert_ne!(&fe1, &fe2);
}

#[test]
fn test_partial_eq_different_prime() {
    let fe1 = FieldElement::from_u64(5, 7);
    let fe2 = FieldElement::from_u64(5, 11);
    assert_ne!(&fe1, &fe2);
}

#[test]
fn test_add_same_field() {
    let fe1 = FieldElement::from_u64(2, 7);
    let fe2 = FieldElement::from_u64(3, 7);
    let result = &fe1 + &fe2;
    assert_eq!(*result.num(), BigUint::from(5u32));
    assert_eq!(*result.prime(), BigUint::from(7u32));
}

#[test]
fn test_add_with_modulo() {
    let fe1 = FieldElement::from_u64(5, 7);
    let fe2 = FieldElement::from_u64(4, 7);
    let result = &fe1 + &fe2;
    assert_eq!(*result.num(), BigUint::from(2u32)); // (5 + 4) % 7 = 2
}

#[test]
#[should_panic(expected = "Cannot add two numbers in different fields")]
fn test_add_different_fields() {
    let fe1 = FieldElement::from_u64(2, 7);
    let fe2 = FieldElement::from_u64(3, 11);
    let _ = &fe1 + &fe2;
}

#[test]
fn test_sub_same_field() {
    let fe1 = FieldElement::from_u64(5, 7);
    let fe2 = FieldElement::from_u64(2, 7);
    let result = &fe1 - &fe2;
    assert_eq!(*result.num(), BigUint::from(3u32));
}

#[test]
fn test_sub_with_wrap_around() {
    let fe1 = FieldElement::from_u64(2, 7);
    let fe2 = FieldElement::from_u64(5, 7);
    let result = &fe1 - &fe2;
    assert_eq!(*result.num(), BigUint::from(4u32)); // (7 + 2 - 5) % 7 = 4
}

#[test]
#[should_panic(expected = "Cannot subtract two numbers in different fields")]
fn test_sub_different_fields() {
    let fe1 = FieldElement::from_u64(5, 7);
    let fe2 = FieldElement::from_u64(2, 11);
    let _ = &fe1 - &fe2;
}

#[test]
fn test_mul_same_field() {
    let fe1 = FieldElement::from_u64(3, 7);
    let fe2 = FieldElement::from_u64(4, 7);
    let result = &fe1 * &fe2;
    assert_eq!(*result.num(), BigUint::from(5u32)); // (3 * 4) % 7 = 5
}

#[test]
fn test_mul_with_modulo() {
    let fe1 = FieldElement::from_u64(5, 7);
    let fe2 = FieldElement::from_u64(6, 7);
    let result = &fe1 * &fe2;
    assert_eq!(*result.num(), BigUint::from(2u32)); // (5 * 6) % 7 = 2
}

#[test]
#[should_panic(expected = "Cannot multiply two numbers in different fields")]
fn test_mul_different_fields() {
    let fe1 = FieldElement::from_u64(3, 7);
    let fe2 = FieldElement::from_u64(4, 11);
    let _ = &fe1 * &fe2;
}

#[test]
fn test_mul_u32() {
    let fe = FieldElement::from_u64(3, 7);
    let result = &fe * 4u32;
    assert_eq!(*result.num(), BigUint::from(5u32)); // (3 * 4) % 7 = 5
}

#[test]
fn test_mul_u32_with_modulo() {
    let fe = FieldElement::from_u64(5, 7);
    let result = &fe * 6u32;
    assert_eq!(*result.num(), BigUint::from(2u32)); // (5 * 6) % 7 = 2
}

#[test]
fn test_div_same_field() {
    let fe1 = FieldElement::from_u64(3, 7);
    let fe2 = FieldElement::from_u64(2, 7);
    let result = &fe1 / &fe2;
    // 3 / 2 in field 7: 3 * 2^(-1) mod 7 = 3 * 4 mod 7 = 5
    assert_eq!(*result.num(), BigUint::from(5u32));
}

#[test]
#[should_panic(expected = "Cannot divide two numbers in different fields")]
fn test_div_different_fields() {
    let fe1 = FieldElement::from_u64(3, 7);
    let fe2 = FieldElement::from_u64(2, 11);
    let _ = &fe1 / &fe2;
}

#[test]
fn test_mod_inverse() {
    let fe = FieldElement::from_u64(3, 7);
    let inverse = fe.inverse();
    // 3^(-1) mod 7 = 5, because 3 * 5 = 15 ≡ 1 (mod 7)
    assert_eq!(*inverse.num(), BigUint::from(5u32));

    // Verify: fe * inverse should equal 1
    let product = &fe * &inverse;
    assert_eq!(*product.num(), BigUint::from(1u32));
}

#[test]
fn test_pow_biguint_ref() {
    let fe = FieldElement::from_u64(3, 7);
    let exp = BigUint::from(2u32);
    let result = fe.pow(&exp);
    assert_eq!(*result.num(), BigUint::from(2u32)); // 3^2 % 7 = 2
}

#[test]
fn test_pow_biguint() {
    let fe = FieldElement::from_u64(3, 7);
    let exp = BigUint::from(3u32);
    let result = fe.pow(exp);
    assert_eq!(*result.num(), BigUint::from(6u32)); // 3^3 % 7 = 6
}

#[test]
fn test_pow_u32() {
    let fe = FieldElement::from_u64(2, 7);
    let result = fe.pow(3u32);
    assert_eq!(*result.num(), BigUint::from(1u32)); // 2^3 % 7 = 1
}

#[test]
fn test_pow_zero() {
    let fe = FieldElement::from_u64(5, 7);
    let result = fe.pow(0u32);
    assert_eq!(*result.num(), BigUint::from(1u32)); // Any number^0 = 1
}

#[test]
fn test_sqrt() {
    // Test with a prime where p ≡ 3 (mod 4), like 7
    let fe = FieldElement::from_u64(4, 7); // 4 is a quadratic residue mod 7
    let sqrt_result = fe.sqrt();

    // Verify that sqrt_result^2 ≡ 4 (mod 7)
    let squared = &sqrt_result * &sqrt_result;
    assert_eq!(*squared.num(), BigUint::from(4u32));
}

#[test]
fn test_clone() {
    let fe1 = FieldElement::from_u64(5, 7);
    let fe2 = fe1.clone();
    assert_eq!(&fe1, &fe2);
    assert_eq!(*fe2.num(), BigUint::from(5u32));
    assert_eq!(*fe2.prime(), BigUint::from(7u32));
}

#[test]
fn test_debug() {
    let fe = FieldElement::from_u64(5, 7);
    let debug_str = format!("{:?}", fe);
    assert!(debug_str.contains("FieldElement"));
    assert!(debug_str.contains("num"));
    assert!(debug_str.contains("prime"));
}

#[test]
fn test_large_numbers() {
    let large_num = BigUint::parse_bytes(b"123456789abcdef", 16).unwrap();
    let large_prime = BigUint::parse_bytes(b"123456789abcdef0", 16).unwrap();

    let fe1 = FieldElement::new(large_num.clone(), large_prime.clone());
    let fe2 = FieldElement::new(large_num.clone(), large_prime.clone());

    // Test operations with large numbers
    let sum = &fe1 + &fe2;
    let expected = (&large_num + &large_num) % &large_prime;
    assert_eq!(*sum.num(), expected);
}

#[test]
fn test_field_arithmetic_properties() {
    let fe1 = FieldElement::from_u64(3, 7);
    let fe2 = FieldElement::from_u64(4, 7);
    let fe3 = FieldElement::from_u64(2, 7);

    // Test commutativity of addition
    let add1 = &fe1 + &fe2;
    let add2 = &fe2 + &fe1;
    assert_eq!(&add1, &add2);

    // Test commutativity of multiplication
    let mul1 = &fe1 * &fe2;
    let mul2 = &fe2 * &fe1;
    assert_eq!(&mul1, &mul2);

    // Test associativity of addition
    let add_assoc1 = &(&fe1 + &fe2) + &fe3;
    let add_assoc2 = &fe1 + &(&fe2 + &fe3);
    assert_eq!(&add_assoc1, &add_assoc2);

    // Test associativity of multiplication
    let mul_assoc1 = &(&fe1 * &fe2) * &fe3;
    let mul_assoc2 = &fe1 * &(&fe2 * &fe3);
    assert_eq!(&mul_assoc1, &mul_assoc2);
}
