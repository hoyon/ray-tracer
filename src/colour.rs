use crate::util;

use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    fn new(r: f32, g: f32, b: f32) -> Self {
        Colour { r, g, b }
    }
}

impl PartialEq for Colour {
    fn eq(&self, other: &Self) -> bool {
        util::float_equality(self.r, other.r)
            && util::float_equality(self.g, other.g)
            && util::float_equality(self.b, other.b)
    }
}

impl ops::Add for Colour {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Colour::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::Sub for Colour {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Colour::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl ops::Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Colour::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl ops::Mul for Colour {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Colour::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new_colour() {
        let colour = Colour::new(1.1, 2.2, 3.3);
        assert_eq!(
            colour,
            Colour {
                r: 1.1,
                g: 2.2,
                b: 3.3
            }
        );
    }

    #[test]
    fn equality_accounts_for_floating_errors() {
        let a = 0.4 + 0.05;
        let b = 0.45;
        assert_ne!(a, b);

        let p = Colour::new(a, a, a);
        assert_eq!(p, Colour::new(b, b, b));
    }

    #[test]
    fn can_add_colours() {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Colour::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn can_subtract_colours() {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Colour::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn can_multiple_colour_by_float() {
        let c = Colour::new(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Colour::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn can_multiple_colours() {
        let c1 = Colour::new(1.0, 0.2, 0.4);
        let c2 = Colour::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Colour::new(0.9, 0.2, 0.04));
    }
}
