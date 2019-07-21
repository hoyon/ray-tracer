type Tuple = (f32, f32, f32, f32);

fn point(x: f32, y: f32, z: f32) -> Tuple {
    (x, y, z, 1.0)
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    (x, y, z, 0.0)
}

fn is_point(tuple: &Tuple) -> bool {
    (tuple.3 - 1.0).abs() < std::f32::EPSILON
}

fn is_vector(tuple: &Tuple) -> bool {
    tuple.3 == 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_makes_point() {
        let p = point(1.3, 1.5, 45.8);
        assert_eq!(p, (1.3, 1.5, 45.8, 1.0))
    }

    #[test]
    fn vector_makes_vector() {
        let v = vector(1.3, 1.5, 45.8);
        assert_eq!(v, (1.3, 1.5, 45.8, 0.0))
    }

    #[test]
    fn is_point_check_works() {
        let p = point(1.3, 1.5, 45.8);
        let v = vector(1.3, 1.5, 45.8);

        assert!(is_point(&p));
        assert!(is_point(&v) == false);
    }

    #[test]
    fn is_vector_check_works() {
        let p = point(1.3, 1.5, 45.8);
        let v = vector(1.3, 1.5, 45.8);

        assert!(is_vector(&p) == false);
        assert!(is_vector(&v));
    }
}
