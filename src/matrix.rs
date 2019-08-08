use crate::util;

#[derive(Clone)]
pub struct Matrix {
    pub rows: u32,
    pub cols: u32,

    data: Vec<f32>,
}

impl Matrix {
    #![allow(clippy::too_many_arguments, clippy::many_single_char_names)]
    pub fn new4x4(a: f32, b: f32, c: f32, d: f32,
                  e: f32, f: f32, g: f32, h: f32,
                  i: f32, j: f32, k: f32, l: f32,
                  m: f32, n: f32, o: f32, p: f32) -> Self {
        Matrix {
            rows: 4,
            cols: 4,
            data: vec![a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]
        }
    }

    pub fn new3x3(a: f32, b: f32, c: f32,
                  d: f32, e: f32, f: f32,
                  g: f32, h: f32, i: f32) -> Self {
        Matrix {
            rows: 3,
            cols: 3,
            data: vec![a, b, c, d, e, f, g, h, i]
        }
    }

    pub fn new2x2(a: f32, b: f32,
                  c: f32, d: f32) -> Self {
        Matrix {
            rows: 2,
            cols: 2,
            data: vec![a, b, c, d]
        }
    }

    pub fn at(&self, r: u32, c: u32) -> f32 {
        assert!(r < self.rows);
        assert!(c < self.cols);

        let idx = r * self.cols + c;
        self.data[idx as usize]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            false
        } else {
            for i in 0..self.data.len() {
                if !util::float_equality(self.data[i], other.data[i]) {
                    return false;
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new4x4_creates_a_matrix() {
        let matrix = Matrix::new4x4(1.0, 2.0, 3.0, 4.0,
                                    5.5, 6.6, 7.5, 8.5,
                                    9.0, 10.0, 11.0, 12.0,
                                    13.5, 14.5, 15.5, 16.5);

        assert_eq!(matrix.at(0, 0), 1.0);
        assert_eq!(matrix.at(0, 3), 4.0);
        assert_eq!(matrix.at(1, 0), 5.5);
        assert_eq!(matrix.at(1, 2), 7.5);
        assert_eq!(matrix.at(2, 2), 11.0);
        assert_eq!(matrix.at(3, 0), 13.5);
        assert_eq!(matrix.at(3, 2), 15.5);
    }

    #[test]
    fn test_new3x3_creates_a_matrix() {
        let matrix = Matrix::new3x3(-3.0, 5.0, 0.0,
                                    1.0, -2.0, -7.0,
                                    0.0, 1.0, 1.0);

        assert_eq!(matrix.at(0, 0), -3.0);
        assert_eq!(matrix.at(1, 1), -2.0);
        assert_eq!(matrix.at(2, 2), 1.0);
    }

    #[test]
    fn test_new2x2_creates_a_matrix() {
        let matrix = Matrix::new2x2(-3.0, 5.0,
                                    1.0, -2.0);

        assert_eq!(matrix.at(0, 0), -3.0);
        assert_eq!(matrix.at(0, 1), 5.0);
        assert_eq!(matrix.at(1, 0), 1.0);
        assert_eq!(matrix.at(1, 1), -2.0);
    }

    #[test]
    fn test_matrix_equality_with_identical_matrices() {
        let matrix = Matrix::new4x4(1.0, 2.0, 3.0, 4.0,
                                    5.5, 6.6, 7.5, 8.5,
                                    9.0, 10.0, 11.0, 12.0,
                                    13.5, 14.5, 15.5, 16.5);

        assert!(matrix == matrix);
    }

    #[test]
    fn test_matrix_equality_accounts_for_floating_errors() {
        let a = 0.4 + 0.05;
        let b = 0.45;
        assert_ne!(a, b);

        let ma = Matrix::new2x2(a, a, a, a);
        let mb = Matrix::new2x2(b, b, b, b);

        assert!(ma == mb);
    }

    #[test]
    fn test_matrix_equality_for_different_matrices() {
        let a = Matrix::new2x2(1.0, 1.0, 1.0, 1.0);
        let b = Matrix::new2x2(2.0, 2.0, 2.0, 2.0);

        assert!(a != b);
    }
}
