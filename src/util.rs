pub fn float_equality(a: f32, b: f32) -> bool {
    (a - b).abs() <= std::f32::EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_equality() {
        let a = 0.4 + 0.05;
        let b = 0.45;
        assert_ne!(a, b);

        assert!(float_equality(a, b));
    }
}
