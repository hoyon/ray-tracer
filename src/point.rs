use std::ops;

#[derive(Debug)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl Tuple {
    fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple{x, y, z, w: 1.0}
    }

    fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple{x, y, z, w: 0.0}
    }

    fn raw(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple{x, y, z, w}
    }

    fn is_point(&self) -> bool {
        (self.w - 1.0).abs() < std::f32::EPSILON
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        float_equality(self.x, other.x) &&
        float_equality(self.y, other.y) &&
        float_equality(self.z, other.z) &&
        float_equality(self.w, other.w)
    }
}

impl ops::Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple::raw(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::raw(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
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

fn float_equality(a: f32, b: f32) -> bool {
    (a - b).abs() < std::f32::EPSILON
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
}
