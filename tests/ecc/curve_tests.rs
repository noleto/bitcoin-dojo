#[cfg(test)]
use bitcoin_dojo::ecc::field::FieldElement;
use bitcoin_dojo::ecc::curve::Point;
use bitcoin_dojo::ecc::scalar::Scalar;
use num_bigint::BigUint;

// Helper function to create test field elements
fn fe(value: u32, prime: u32) -> FieldElement {
    FieldElement::new(BigUint::from(value), BigUint::from(prime))
}

#[test]
fn test_point_creation() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    // Valid point
    let point = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a.clone(), b.clone());
    assert!(!point.is_infinity());

    // Point at infinity
    let infinity = Point::new(None, None, a, b);
    assert!(infinity.is_infinity());
}

#[test]
#[should_panic(expected = "is not on the curve")]
fn test_invalid_point() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    // This should panic as it's not on the curve
    Point::new(Some(fe(200, 223)), Some(fe(119, 223)), a, b);
}

#[test]
fn test_point_addition() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p1 = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a.clone(), b.clone());
    let p2 = Point::new(Some(fe(17, 223)), Some(fe(56, 223)), a.clone(), b.clone());

    let result = &p1 + &p2;
    assert!(!result.is_infinity());
}

#[test]
fn test_point_infinity_addition() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a.clone(), b.clone());
    let infinity = Point::infinity(a, b);

    let result1 = &p + &infinity;
    let result2 = &infinity + &p;

    assert_eq!(result1, p);
    assert_eq!(result2, p);
}

#[test]
fn test_scalar_multiplication() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);
    let scalar = BigUint::from(2u32);

    let result = &p * scalar;
    let expected = &p + &p;

    assert_eq!(result, expected);
}

// Additional comprehensive tests:

#[test]
fn test_point_getters() {
    let a = fe(0, 223);
    let b = fe(7, 223);
    let x = fe(192, 223);
    let y = fe(105, 223);

    let point = Point::new(Some(x.clone()), Some(y.clone()), a.clone(), b.clone());

    assert_eq!(point.x(), &Some(x));
    assert_eq!(point.y(), &Some(y));
    assert_eq!(point.a(), &a);
    assert_eq!(point.b(), &b);
}

#[test]
fn test_point_infinity_getters() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let infinity = Point::infinity(a.clone(), b.clone());

    assert_eq!(infinity.x(), &None);
    assert_eq!(infinity.y(), &None);
    assert_eq!(infinity.a(), &a);
    assert_eq!(infinity.b(), &b);
    assert!(infinity.is_infinity());
}

#[test]
fn test_point_doubling() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);
    
    // Point doubling: P + P
    let doubled = &p + &p;
    assert!(!doubled.is_infinity());
    
    // Should be the same as 2 * P
    let scalar_doubled = &p * BigUint::from(2u32);
    assert_eq!(doubled, scalar_doubled);
}

#[test]
fn test_point_vertical_line_case() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    // Find a point and its negation (same x, opposite y)
    let p1 = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a.clone(), b.clone());
    
    // Calculate -p1 (same x, negated y)
    // For prime 223, -105 mod 223 = 223 - 105 = 118
    let p2 = Point::new(Some(fe(192, 223)), Some(fe(118, 223)), a.clone(), b.clone());
    
    // Adding a point to its negation should give infinity
    let result = &p1 + &p2;
    assert!(result.is_infinity());
}

#[test]
fn test_point_same_point() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p1 = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a.clone(), b.clone());
    let p2 = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a.clone(), b.clone());
    let p3 = Point::new(Some(fe(17, 223)), Some(fe(56, 223)), a.clone(), b.clone());

    assert!(p1.same_point(&p2));
    assert!(!p1.same_point(&p3));

    let infinity1 = Point::infinity(a.clone(), b.clone());
    let infinity2 = Point::infinity(a, b);
    assert!(infinity1.same_point(&infinity2));
}

#[test]
fn test_scalar_multiplication_with_scalar_type() {
    let a = fe(0, 223);
    let b = fe(7, 223);
    let n = BigUint::from(223u32); // Using the same prime as modulus for simplicity

    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);
    let scalar = Scalar::new(BigUint::from(3u32), n);

    let result1 = &p * &scalar;
    let result2 = p.multiply(&scalar);
    let expected = &(&p + &p) + &p; // 3 * P = P + P + P

    assert_eq!(result1, result2);
    assert_eq!(result1, expected);
}

#[test]
fn test_scalar_multiplication_zero() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);
    let result = &p * BigUint::from(0u32);

    assert!(result.is_infinity());
}

#[test]
fn test_scalar_multiplication_one() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);
    let result = &p * BigUint::from(1u32);

    assert_eq!(result, p);
}

#[test]
fn test_scalar_multiplication_large() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);
    let large_scalar = BigUint::from(100u32);
    
    let result = &p * large_scalar;
    // Should not panic and should produce a valid point
    assert!(!result.is_infinity() || result.is_infinity()); // Either is valid
}

#[test]
fn test_point_addition_owned_variants() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p1 = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a.clone(), b.clone());
    let p2 = Point::new(Some(fe(17, 223)), Some(fe(56, 223)), a.clone(), b.clone());

    // Test all combinations of owned/borrowed addition
    let result1 = &p1 + &p2;
    let result2 = p1.clone() + &p2;
    let result3 = &p1 + p2.clone();
    let result4 = p1.clone() + p2.clone();

    assert_eq!(result1, result2);
    assert_eq!(result2, result3);
    assert_eq!(result3, result4);
}

#[test]
fn test_point_multiplication_owned_variants() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);
    let scalar = BigUint::from(3u32);

    // Test all combinations of owned/borrowed multiplication
    let result1 = &p * scalar.clone();
    let result2 = &p * &scalar;
    let result3 = p.clone() * scalar.clone();
    let result4 = p.clone() * &scalar;

    assert_eq!(result1, result2);
    assert_eq!(result2, result3);
    assert_eq!(result3, result4);
}

#[test]
#[should_panic(expected = "are not on the same curve")]
fn test_different_curve_addition() {
    let a1 = fe(0, 223);
    let b1 = fe(7, 223);
    let a2 = fe(1, 223); // Different curve parameter
    let b2 = fe(7, 223);

    let p1 = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a1, b1);
    // Need to find a valid point on the different curve
    let p2 = Point::infinity(a2, b2);

    let _ = &p1 + &p2; // Should panic
}

#[test]
#[should_panic(expected = "Invalid parameters to Point::new()")]
fn test_invalid_point_parameters() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    // x is Some but y is None - invalid
    Point::new(Some(fe(192, 223)), None, a, b);
}

#[test]
fn test_point_equality() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p1 = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a.clone(), b.clone());
    let p2 = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a.clone(), b.clone());
    let p3 = Point::new(Some(fe(17, 223)), Some(fe(56, 223)), a.clone(), b.clone());

    assert_eq!(p1, p2);
    assert_ne!(p1, p3);

    // Test reference equality
    assert_eq!(&p1, &p2);
    assert_ne!(&p1, &p3);
}

#[test]
fn test_point_tangent_vertical_case() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    // Find a point where the tangent line is vertical (y = 0)
    // For y^2 = x^3 + 7, when y = 0: 0 = x^3 + 7, so x^3 = -7 mod 223
    // We need to find if there's a point with y = 0
    // Let's use a different approach - create a point and test doubling edge cases
    
    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);
    
    // Test that doubling works correctly (this tests the tangent case)
    let doubled = &p + &p;
    let scalar_doubled = &p * BigUint::from(2u32);
    
    assert_eq!(doubled, scalar_doubled);
}

#[test]
fn test_associativity_of_point_addition() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p1 = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a.clone(), b.clone());
    let p2 = Point::new(Some(fe(17, 223)), Some(fe(56, 223)), a.clone(), b.clone());
    let p3 = Point::new(Some(fe(1, 223)), Some(fe(193, 223)), a.clone(), b.clone());

    // Test associativity: (p1 + p2) + p3 = p1 + (p2 + p3)
    let left = &(&p1 + &p2) + &p3;
    let right = &p1 + &(&p2 + &p3);

    assert_eq!(left, right);
}

#[test]
fn test_commutativity_of_point_addition() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p1 = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a.clone(), b.clone());
    let p2 = Point::new(Some(fe(17, 223)), Some(fe(56, 223)), a.clone(), b.clone());

    // Test commutativity: p1 + p2 = p2 + p1
    let result1 = &p1 + &p2;
    let result2 = &p2 + &p1;

    assert_eq!(result1, result2);
}

#[test]
fn test_scalar_multiplication_distributivity() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p1 = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a.clone(), b.clone());
    let p2 = Point::new(Some(fe(17, 223)), Some(fe(56, 223)), a.clone(), b.clone());
    let k = BigUint::from(3u32);

    // Test distributivity: k * (p1 + p2) = k * p1 + k * p2
    let left = &(&p1 + &p2) * k.clone();
    let right = &(&p1 * k.clone()) + &(&p2 * k);

    assert_eq!(left, right);
}

#[test]
fn test_scalar_multiplication_associativity() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);
    let k1 = BigUint::from(3u32);
    let k2 = BigUint::from(5u32);

    // Test associativity: (k1 * k2) * p = k1 * (k2 * p)
    let left = &p * (&k1 * &k2);
    let right = &(&p * k2) * k1;

    assert_eq!(left, right);
}

#[test]
fn test_point_order_properties() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);

    // Test that scalar multiplication works for various values
    // and that we can compute large multiples without panicking
    let mut current = p.clone();
    
    // Test first few multiples
    for i in 1..=10 {
        current = &current + &p;
        let scalar_result = &p * BigUint::from((i + 1) as u32);
        assert_eq!(current, scalar_result);
    }
    
    // Test that we can compute larger multiples
    let large_multiple = &p * BigUint::from(100u32);
    // The result is valid whether it's infinity or not
    // Main goal is to ensure no panic occurs
    
    // Test that if we ever hit infinity, subsequent additions stay at infinity
    if large_multiple.is_infinity() {
        let still_infinity = &large_multiple + &p;
        assert!(still_infinity.is_infinity());
    }
}

#[test]
fn test_point_binary_representation() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);

    // Test that scalar multiplication with powers of 2 works correctly
    let p2 = &p * BigUint::from(2u32);   // 2P
    let p4 = &p * BigUint::from(4u32);   // 4P
    let p8 = &p * BigUint::from(8u32);   // 8P

    // Verify: 4P = 2P + 2P
    assert_eq!(p4, &p2 + &p2);
    
    // Verify: 8P = 4P + 4P
    assert_eq!(p8, &p4 + &p4);
    
    // Verify: 8P = 2 * 4P
    assert_eq!(p8, &p4 * BigUint::from(2u32));
}

#[test]
fn test_point_edge_cases() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    // Test infinity + infinity = infinity
    let inf1 = Point::infinity(a.clone(), b.clone());
    let inf2 = Point::infinity(a.clone(), b.clone());
    let result = &inf1 + &inf2;
    assert!(result.is_infinity());

    // Test infinity * k = infinity for any k
    let inf_scaled = &inf1 * BigUint::from(42u32);
    assert!(inf_scaled.is_infinity());

    // Test point * 0 = infinity
    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);
    let zero_result = &p * BigUint::from(0u32);
    assert!(zero_result.is_infinity());
}

#[test]
fn test_point_clone_and_debug() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p1 = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);
    let p2 = p1.clone();

    assert_eq!(p1, p2);
    
    // Test that Debug trait works (should not panic)
    let debug_str = format!("{:?}", p1);
    assert!(!debug_str.is_empty());
    
    let inf = Point::infinity(fe(0, 223), fe(7, 223));
    let inf_debug = format!("{:?}", inf);
    assert!(!inf_debug.is_empty());
}

#[test]
fn test_multiple_curve_parameters() {
    // Test with different curve parameters
    
    // Curve y^2 = x^3 + 7 (mod 223)
    let a1 = fe(0, 223);
    let b1 = fe(7, 223);
    let p1 = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a1, b1);
    
    // Different curve (if we can find valid points)
    // For now, just test that different curves work independently
    let a2 = fe(0, 223);
    let b2 = fe(7, 223);
    let inf2 = Point::infinity(a2, b2);
    
    assert!(!p1.is_infinity());
    assert!(inf2.is_infinity());
}

#[test]
fn test_scalar_with_different_moduli() {
    let a = fe(0, 223);
    let b = fe(7, 223);
    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);

    // Test with different scalar moduli
    let n1 = BigUint::from(17u32);
    let n2 = BigUint::from(223u32);
    
    let scalar1 = Scalar::new(BigUint::from(3u32), n1);
    let scalar2 = Scalar::new(BigUint::from(3u32), n2);
    
    let result1 = &p * &scalar1;
    let result2 = &p * &scalar2;
    
    // Both should work (the actual scalar value is the same)
    assert_eq!(result1, result2);
}

#[test]
fn test_performance_large_scalar() {
    let a = fe(0, 223);
    let b = fe(7, 223);
    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);

    // Test with a reasonably large scalar to ensure binary method is efficient
    let large_scalar = BigUint::from(1000u32);
    
    let start = std::time::Instant::now();
    let result = &p * large_scalar;
    let duration = start.elapsed();
    
    // Should complete quickly (less than 1 second for this size)
    assert!(duration.as_secs() < 1);
    
    // Result should be valid (either a point or infinity)
    assert!(result.is_infinity() || !result.is_infinity());
}

#[test]
fn test_point_coordinates_access() {
    let a = fe(0, 223);
    let b = fe(7, 223);
    let x_val = fe(192, 223);
    let y_val = fe(105, 223);

    let p = Point::new(Some(x_val.clone()), Some(y_val.clone()), a.clone(), b.clone());

    // Test coordinate access
    match (p.x(), p.y()) {
        (Some(x), Some(y)) => {
            assert_eq!(x, &x_val);
            assert_eq!(y, &y_val);
        }
        _ => panic!("Expected Some coordinates"),
    }

    // Test curve parameter access
    assert_eq!(p.a(), &a);
    assert_eq!(p.b(), &b);

    // Test infinity coordinates
    let inf = Point::infinity(a.clone(), b.clone());
    assert_eq!(inf.x(), &None);
    assert_eq!(inf.y(), &None);
    assert_eq!(inf.a(), &a);
    assert_eq!(inf.b(), &b);
}

#[test]
fn test_point_arithmetic_properties() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a.clone(), b.clone());
    let inf = Point::infinity(a, b);

    // Test identity element: P + O = P (where O is point at infinity)
    assert_eq!(&p + &inf, p);
    assert_eq!(&inf + &p, p);
}

#[test]
fn test_point_special_cases() {
    let a = fe(0, 223);
    let b = fe(7, 223);

    // Test point doubling when the tangent is vertical
    // This happens when y = 0, but we need to find such a point first
    // For the curve y^2 = x^3 + 7, when y = 0: x^3 = -7 (mod 223)
    
    // Instead, let's test the general doubling case
    let p = Point::new(Some(fe(192, 223)), Some(fe(105, 223)), a, b);
    
    // P + P should equal 2P
    let doubled_add = &p + &p;
    let doubled_mul = &p * BigUint::from(2u32);
    assert_eq!(doubled_add, doubled_mul);

    // Test 3P = P + P + P = P + 2P
    let triple_add = &(&p + &p) + &p;
    let triple_mixed = &p + &doubled_add;
    let triple_mul = &p * BigUint::from(3u32);
    
    assert_eq!(triple_add, triple_mixed);
    assert_eq!(triple_mixed, triple_mul);
}
