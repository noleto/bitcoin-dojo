use crate::ecc::field::{FieldElement, Pow};
use crate::ecc::scalar::Scalar;
use num_bigint::BigUint;
use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Point(x={},y={})",
            self.x().as_ref().unwrap(),
            self.y().as_ref().unwrap()
        )
    }
}

impl Point {
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>) -> Self {
        match (x, y) {
            (Some(x), Some(y)) => {
                let x_cubed = x.pow(BigUint::from(3u32));
                let right_side = &x_cubed + FieldElement::secp256k1_fe_b();
                let y_squared = y.pow(BigUint::from(2u32));
                if y_squared != right_side {
                    panic!("is not on the secp256k1 curve")
                }
                Self {
                    x: Some(x),
                    y: Some(y),
                }
            }
            (None, None) => Self { x: None, y: None },
            _ => {
                panic!("Invalid parameters to Point::new()")
            }
        }
    }

    pub fn x(&self) -> &Option<FieldElement> {
        &self.x
    }

    pub fn y(&self) -> &Option<FieldElement> {
        &self.y
    }

    pub fn a(&self) -> &FieldElement {
        FieldElement::secp256k1_fe_a()
    }

    pub fn b(&self) -> &FieldElement {
        FieldElement::secp256k1_fe_b()
    }

    // Returns the point at infinity
    pub fn infinity() -> Point {
        Self { x: None, y: None }
    }

    // Returns the point at infinity with same curve parameters
    pub fn new_infinity(&self) -> Point {
        Self::infinity()
    }

    pub fn is_infinity(&self) -> bool {
        self.x().is_none() && self.y().is_none()
    }

    // Scalar multiplication using the Scalar type
    pub fn multiply(&self, scalar: &Scalar) -> Point {
        // Convert scalar to BigUint and use existing multiplication
        let coef = scalar.value().clone();
        self * coef
    }

    // Check if this point is the same as another (ignoring curve parameters)
    pub fn same_point(&self, other: &Point) -> bool {
        match (&self.x, &other.x, &self.y, &other.y) {
            (Some(x1), Some(x2), Some(y1), Some(y2)) => x1 == x2 && y1 == y2,
            (None, None, None, None) => true,
            _ => false,
        }
    }

    pub fn generator() -> Point {
        Point {
            x: Some(FieldElement::secp256k1_fe_gx().clone()),
            y: Some(FieldElement::secp256k1_fe_gy().clone()),
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let x_eq = match (&self.x, &other.x) {
            (Some(x1), Some(x2)) => x1 == x2,
            (None, None) => true,
            _ => false,
        };
        let y_eq = match (&self.y, &other.y) {
            (Some(y1), Some(y2)) => y1 == y2,
            (None, None) => true,
            _ => false,
        };
        x_eq && y_eq
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        &self + &other
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        &self + other
    }
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        self + &other
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        match (self.x(), self.y(), other.x(), other.y()) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) => {
                // Point addition for when p1 == p2 has a diffente formula
                let (s_num, s_den) = if self == other {
                    (&x1.pow(2u32) * 3u32, y1 * 2u32)
                } else {
                    (y1 - y2, x1 - x2)
                };
                // case for vertical or horizontal line
                if s_num.is_zero() || s_den.is_zero() {
                    self.new_infinity()
                } else {
                    // slope of the tangent
                    let slope = &s_num / &s_den;
                    let new_x = &(&slope.pow(2) - x1) - x2;
                    let new_y = &(&slope * &(x1 - &new_x)) - y1;
                    Point::new(Some(new_x), Some(new_y))
                }
            }
            (None, _, _, _) => other.clone(),
            (_, _, None, _) => self.clone(),
            _ => unreachable!("x and y must both be Some or both be None"),
        }
    }
}

impl Mul<BigUint> for Point {
    type Output = Point;

    fn mul(self, coefficient: BigUint) -> Self::Output {
        &self * coefficient
    }
}

impl Mul<&BigUint> for Point {
    type Output = Point;

    fn mul(self, coefficient: &BigUint) -> Self::Output {
        &self * coefficient.clone()
    }
}

impl Mul<BigUint> for &Point {
    type Output = Point;

    // Scalar multiplication using binary expansion
    fn mul(self, coefficient: BigUint) -> Self::Output {
        let mut coef = coefficient;
        let mut current = self.clone();
        let mut result = self.new_infinity();

        while coef > BigUint::from(0u32) {
            // Check if the rightmost bit is 1
            if &coef & BigUint::from(1u32) == BigUint::from(1u32) {
                result = &result + &current;
            }
            // Double the current point
            current = &current + &current;
            // Right shift the coefficient
            coef >>= 1;
        }
        result
    }
}

impl Mul<&BigUint> for &Point {
    type Output = Point;

    fn mul(self, coefficient: &BigUint) -> Self::Output {
        self * coefficient.clone()
    }
}

// Implement scalar multiplication with Scalar type
impl Mul<Scalar> for Point {
    type Output = Point;

    fn mul(self, scalar: Scalar) -> Self::Output {
        &self * scalar.value().clone()
    }
}

impl Mul<&Scalar> for Point {
    type Output = Point;

    fn mul(self, scalar: &Scalar) -> Self::Output {
        &self * scalar.value().clone()
    }
}

impl Mul<Scalar> for &Point {
    type Output = Point;

    fn mul(self, scalar: Scalar) -> Self::Output {
        self * scalar.value().clone()
    }
}

impl Mul<&Scalar> for &Point {
    type Output = Point;

    fn mul(self, scalar: &Scalar) -> Self::Output {
        self * scalar.value().clone()
    }
}
