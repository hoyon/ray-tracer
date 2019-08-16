use crate::{Tuple, util};
use std::ops;

#[derive(Clone, Debug)]
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

    fn set_cell(&mut self, r: u32, c: u32, value: f32) {
        let idx = r * self.cols + c;
        self.data[idx as usize] = value;
    }

    pub fn transpose(&self) -> Matrix {
        assert!(self.rows == self.cols, "Can only transpose square matrices");

        let mut ret = self.clone();

        for r in 0..self.rows {
            for c in 0..self.cols {
                ret.set_cell(r, c, self.at(c, r));
            }
        }
        ret
    }

    pub fn determinant(&self) -> f32 {
        if self.has_size(2) {
            self.at(0, 0) * self.at(1, 1) - self.at(0, 1) * self.at(1, 0)
        } else {
            let mut det = 0.0;
            for i in 0..self.cols {
                det += self.at(0, i) * self.cofactor(0, i)
            }

            det
        }
    }

    pub fn submatrix(&self, row: u32, col: u32) -> Matrix {
        assert!(self.rows > row && self.cols > col);

        let mut result = self.clone();

        // remove column
        let index = col;
        for i in 0..result.rows {
            let original_index = index + i * result.cols;
            // index changes every time item is deleted
            let adjusted_index = original_index - i;
            result.data.remove(adjusted_index as usize);
        }
        result.cols -= 1;

        // remove row
        let index = row * result.cols;
        for _ in 0..result.cols {
            result.data.remove(index as usize);
        }
        result.rows -= 1;

        result
    }

    pub fn minor(&self, row: u32, col: u32) -> f32 {
        let sub = self.submatrix(row, col);
        sub.determinant()
    }

    pub fn cofactor(&self, row: u32, col: u32) -> f32 {
        let minor = self.minor(row, col);

        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn row(&self, r: u32) -> Tuple {
        assert!(r < 4);
        assert!(self.has_size(4), "Can only get row of 4x4 matrices");

        Tuple::raw(self.at(r, 0), self.at(r, 1), self.at(r, 2), self.at(r, 3))
    }

    fn col(&self, c: u32) -> Tuple {
        assert!(c < 4);
        assert!(self.has_size(4), "Can only get col of 4x4 matrices");

        Tuple::raw(self.at(0, c), self.at(1, c), self.at(2, c), self.at(3, c))
    }

    fn has_size(&self, size: u32) -> bool {
        self.rows == size && self.cols == size
    }

    pub fn identity() -> Matrix {
        Matrix::new4x4(1.0, 0.0, 0.0, 0.0,
                       0.0, 1.0, 0.0, 0.0,
                       0.0, 0.0, 1.0, 0.0,
                       0.0, 0.0, 0.0, 1.0)
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

impl ops::Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Matrix) -> Self::Output {
        assert!(self.has_size(4) && rhs.has_size(4), "Can only multiply 4x4 matrices");

        let mut ret = self.clone();

        for row in 0..=3 {
            for col in 0..=3 {
                let value = Tuple::dot(&self.row(row), &rhs.col(col));
                ret.set_cell(row, col, value);
            }
        }

        ret
    }
}

impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        assert!(self.rows == 4 && self.cols == 4, "Can only multiply 4x4 matrix with tuple");

        let mut ret = rhs;

        ret.x = Tuple::dot(&rhs, &self.row(0));
        ret.y = Tuple::dot(&rhs, &self.row(1));
        ret.z = Tuple::dot(&rhs, &self.row(2));
        ret.w = Tuple::dot(&rhs, &self.row(3));

        ret
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

    #[test]
    fn test_can_multiple_4x4_matrices() {
        let a = Matrix::new4x4(1.0, 2.0, 3.0, 4.0,
                               5.0, 6.0, 7.0, 8.0,
                               9.0, 8.0, 7.0, 6.0,
                               5.0, 4.0, 3.0, 2.0);

        let b = Matrix::new4x4(-2.0, 1.0, 2.0, 3.0,
                               3.0, 2.0, 1.0, -1.0,
                               4.0, 3.0, 6.0, 5.0,
                               1.0, 2.0, 7.0, 8.0);

        let expected = Matrix::new4x4(20.0, 22.0, 50.0, 48.0,
                                      44.0, 54.0, 114.0, 108.0,
                                      40.0, 58.0, 110.0, 102.0,
                                      16.0, 26.0, 46.0, 42.0);

        assert_eq!(expected, a * b);
    }

    #[test]
    #[should_panic]
    fn test_cannot_multiply_other_matrix_sizes() {
        let m = Matrix::new2x2(1.0, 1.0, 1.0, 1.0);
        let _ = m.clone() * m;
    }

    #[test]
    fn test_can_multiple_matrix_4x4_with_tuple() {
        let a = Matrix::new4x4(1.0, 2.0, 3.0, 4.0,
                               2.0, 4.0, 4.0, 2.0,
                               8.0, 6.0, 4.0, 1.0,
                               0.0, 0.0, 0.0, 1.0);

        let b = Tuple::raw(1.0, 2.0, 3.0, 1.0);

        assert_eq!(a * b, Tuple::raw(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn test_multiplying_matrix_with_identity() {
        let m = Matrix::new4x4(1.0, 2.0, 3.0, 4.0,
                               2.0, 4.0, 4.0, 2.0,
                               8.0, 6.0, 4.0, 1.0,
                               0.0, 0.0, 0.0, 1.0);

        assert_eq!(m.clone(), m * Matrix::identity());
    }

    #[test]
    fn test_multiplying_tuple_with_identity() {
        let t = Tuple::raw(1.0, 2.0, 3.0, 1.0);

        assert_eq!(t, Matrix::identity() * t);
    }

    #[test]
    fn test_transpose_works() {
        let m = Matrix::new4x4(1.0, 2.0, 3.0, 4.0,
                               2.0, 4.0, 4.0, 2.0,
                               8.0, 6.0, 4.0, 1.0,
                               0.0, 0.0, 0.0, 1.0);

        let e = Matrix::new4x4(1.0, 2.0, 8.0, 0.0,
                               2.0, 4.0, 6.0, 0.0,
                               3.0, 4.0, 4.0, 0.0,
                               4.0, 2.0, 1.0, 1.0);

        assert_eq!(e, m.transpose());
    }

    #[test]
    fn test_transpose_identity() {
        assert_eq!(Matrix::identity(), Matrix::identity().transpose());
    }

    #[test]
    fn test_determinant_of_2x2_matrix() {
        let m = Matrix::new2x2(1.0, 5.0,
                               -3.0, 2.0);

        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn test_determinant_of_3x3_matrix() {
        let m = Matrix::new3x3(1.0, 2.0, 6.0,
                               -5.0, 8.0, -4.0,
                               2.0, 6.0, 4.0);

        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn test_determinant_of_4x4_matrix() {
        let m = Matrix::new4x4(-2.0, -8.0, 3.0, 5.0,
                               -3.0, 1.0, 7.0, 3.0,
                               1.0, 2.0, -9.0, 6.0,
                               -6.0, 7.0, 7.0, -9.0);

        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn test_submatrix_of_3x3() {
        let matrix = Matrix::new3x3(-3.0, 5.0, 0.0,
                                    1.0, -2.0, -7.0,
                                    0.0, 1.0, 1.0);

        let expected = Matrix::new2x2(1.0, -2.0,
                                      0.0, 1.0);

        assert_eq!(expected, matrix.submatrix(0, 2));

        let matrix = Matrix::new3x3(-3.0, 5.0, 0.0,
                                    1.0, -2.0, -7.0,
                                    0.0, 1.0, 1.0);

        let expected = Matrix::new2x2(-3.0, 5.0,
                                      0.0, 1.0);

        assert_eq!(expected, matrix.submatrix(1, 2));
    }

    #[test]
    fn test_submatrix_of_4x4() {
        let matrix = Matrix::new4x4(1.0, 2.0, 3.0, 4.0,
                                    2.0, 4.0, 4.0, 2.0,
                                    8.0, 6.0, 4.0, 1.0,
                                    0.0, 0.0, 0.0, 1.0);

        let expected = Matrix::new3x3(1.0, 3.0, 4.0,
                                      2.0, 4.0, 2.0,
                                      0.0, 0.0, 1.0);

        assert_eq!(expected, matrix.submatrix(2, 1));
    }

    #[test]
    fn test_minor_of_3x3() {
        let matrix = Matrix::new3x3(3.0, 5.0, 0.0,
                                    2.0, -1.0, -7.0,
                                    6.0, -1.0, 5.0);

        let sub = matrix.submatrix(1, 0);

        assert_eq!(sub.determinant(), 25.0);
        assert_eq!(matrix.minor(1, 0), 25.0);
    }

    #[test]
    fn test_cofactor_of_3x3() {
        let matrix = Matrix::new3x3(3.0, 5.0, 0.0,
                                    2.0, -1.0, -7.0,
                                    6.0, -1.0, 5.0);

        assert_eq!(matrix.minor(0, 0), -12.0);
        assert_eq!(matrix.cofactor(0, 0), -12.0);
        assert_eq!(matrix.minor(1, 0), 25.0);
        assert_eq!(matrix.cofactor(1, 0), -25.0);
    }
}