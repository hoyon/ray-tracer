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
}
