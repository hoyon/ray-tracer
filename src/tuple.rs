use std::fmt;
use std::ops;
use crate::util;

#[derive(Clone, Copy)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn raw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn is_point(&self) -> bool {
        (self.w - 1.0).abs() < std::f32::EPSILON
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f32 {
        let sum = (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w);
        sum.sqrt()
    }

    pub fn normalise(&self) -> Self {
        let magnitude = self.magnitude();
        Tuple::raw(
            self.x / magnitude,
            self.y / magnitude,
            self.z / magnitude,
            self.w / magnitude,
        )
    }

    pub fn dot(a: &Self, b: &Self) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    pub fn cross(a: &Self, b: &Self) -> Self {
        Tuple::vector(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        util::float_equality(self.x, other.x)
            && util::float_equality(self.y, other.y)
            && util::float_equality(self.z, other.z)
            && util::float_equality(self.w, other.w)
    }
}

impl ops::Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple::raw(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::raw(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let zero = Tuple::raw(0.0, 0.0, 0.0, 0.0);
        zero - self
    }
}

impl ops::Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Tuple::raw(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl ops::Div<f32> for Tuple {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Tuple::raw(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl fmt::Debug for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {:?}, {:?})",
            self.x, self.y, self.z, self.w
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_makes_point() {
        let p = Tuple::point(1.3, 1.5, 45.8);
        assert_eq!(p, Tuple::raw(1.3, 1.5, 45.8, 1.0))
    }

    #[test]
    fn vector_makes_vector() {
        let v = Tuple::vector(1.3, 1.5, 45.8);
        assert_eq!(v, Tuple::raw(1.3, 1.5, 45.8, 0.0))
    }

    #[test]
    fn is_point_check_works() {
        let p = Tuple::point(1.3, 1.5, 45.8);
        let v = Tuple::vector(1.3, 1.5, 45.8);

        assert!(p.is_point());
        assert!(v.is_point() == false);
    }

    #[test]
    fn vector_check_works() {
        let p = Tuple::point(1.3, 1.5, 45.8);
        let v = Tuple::vector(1.3, 1.5, 45.8);

        assert!(p.is_vector() == false);
        assert!(v.is_vector());
    }

    #[test]
    fn equality_accounts_for_floating_errors() {
        let a = 0.4 + 0.05;
        let b = 0.45;
        assert_ne!(a, b);

        let p = Tuple::point(a, a, a);
        assert_eq!(p, Tuple::raw(b, b, b, 1.0));
    }

    #[test]
    fn can_add_two_tuples() {
        let a = Tuple::point(3.0, -2.0, 5.0);
        let b = Tuple::vector(-2.0, 3.0, 1.0);
        assert_eq!(a + b, Tuple::point(1.0, 1.0, 6.0));
    }

    #[test]
    fn can_subtract_two_points() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::point(5.0, 6.0, 7.0);
        assert_eq!(a - b, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn can_subtract_vector_from_point() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(a - b, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn can_subtract_two_vectors() {
        let a = Tuple::vector(3.0, 2.0, 1.0);
        let b = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(a - b, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_zero() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn can_negate_a_tuple() {
        let t = Tuple::raw(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-t, Tuple::raw(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn can_multiple_a_tuple_by_scalar() {
        let t = Tuple::raw(1.0, -2.0, 3.0, -4.0);
        assert_eq!(t * 3.5, Tuple::raw(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn can_multiple_a_tuple_by_fraction() {
        let t = Tuple::raw(1.0, -2.0, 3.0, -4.0);
        assert_eq!(t * 0.5, Tuple::raw(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn can_divide_a_tuple_by_fraction() {
        let t = Tuple::raw(1.0, -2.0, 3.0, -4.0);
        assert_eq!(t / 2.0, Tuple::raw(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn correctly_calculates_magnitude() {
        assert_eq!(Tuple::vector(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Tuple::vector(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(Tuple::vector(1.0, 2.0, 3.0).magnitude(), 14_f32.sqrt());
        assert_eq!(Tuple::vector(-1.0, -2.0, -3.0).magnitude(), 14_f32.sqrt());
    }

    #[test]
    fn normalise_works_for_single_direction() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalise(), Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalise_works_for_complex_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(
            v.normalise(),
            Tuple::vector(0.26726124, 0.5345225, 0.8017837)
        );
    }

    #[test]
    fn magnitude_of_normalised_vector_is_one() {
        let v = Tuple::vector(1.0, 2.0, 2.0);
        assert_eq!(v.normalise().magnitude(), 1.0);
    }

    #[test]
    fn dot_product_works() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(Tuple::dot(&a, &b), 20.0);
    }

    #[test]
    fn cross_product_works() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(Tuple::cross(&a, &b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(Tuple::cross(&b, &a), Tuple::vector(1.0, -2.0, 1.0));
    }
}
