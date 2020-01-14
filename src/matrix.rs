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

    pub fn invert(&self) -> Matrix {
        let det = self.determinant();
        assert!(det != 0.0);

        let mut ret = self.clone();

        for r in 0..ret.rows {
            for c in 0..ret.cols {
                let co = self.cofactor(r, c);

                ret.set_cell(c, r, co / det);
            }
        }

        ret
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

    pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
        let mut base = Matrix::identity();
        base.set_cell(0, 3, x);
        base.set_cell(1, 3, y);
        base.set_cell(2, 3, z);

        base
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Matrix {
        let mut base = Matrix::identity();
        base.set_cell(0, 0, x);
        base.set_cell(1, 1, y);
        base.set_cell(2, 2, z);

        base
    }

    pub fn rotation_x(rad: f32) -> Matrix {
        let mut base = Matrix::identity();
        base.set_cell(1, 1, rad.cos());
        base.set_cell(2, 1, rad.sin());
        base.set_cell(1, 2, -rad.sin());
        base.set_cell(2, 2, rad.cos());

        base
    }

    pub fn rotation_y(rad: f32) -> Matrix {
        let mut base = Matrix::identity();
        base.set_cell(0, 0, rad.cos());
        base.set_cell(0, 2, rad.sin());
        base.set_cell(2, 0, -rad.sin());
        base.set_cell(2, 2, rad.cos());

        base
    }

    pub fn rotation_z(rad: f32) -> Matrix {
        let mut base = Matrix::identity();
        base.set_cell(0, 0, rad.cos());
        base.set_cell(0, 1, -rad.sin());
        base.set_cell(1, 0, rad.sin());
        base.set_cell(1, 1, rad.cos());

        base
    }

    pub fn shearing(x_y: f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Matrix {
        let mut base = Matrix::identity();
        base.set_cell(0, 1, x_y);
        base.set_cell(0, 2, x_z);
        base.set_cell(1, 0, y_x);
        base.set_cell(1, 2, y_z);
        base.set_cell(2, 0, z_x);
        base.set_cell(2, 1, z_y);

        base
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Matrix {
        Matrix::translation(x, y, z) * self
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Matrix {
        Matrix::scaling(x, y, z) * self
    }

    pub fn rotate_x(&self, radians: f32) -> Matrix {
        Matrix::rotation_x(radians) * self
    }

    pub fn rotate_y(&self, radians: f32) -> Matrix {
        Matrix::rotation_y(radians) * self
    }

    pub fn rotate_z(&self, radians: f32) -> Matrix {
        Matrix::rotation_z(radians) * self
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
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        &self * &rhs
    }
}

impl ops::Mul<&Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        &self * rhs
    }
}

impl ops::Mul<Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        self * &rhs
    }
}

impl ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
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
        &self * &rhs
    }
}

impl ops::Mul<&Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: &Tuple) -> Self::Output {
        &self * rhs
    }
}

impl ops::Mul<Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        self * &rhs
    }
}

impl ops::Mul<&Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, rhs: &Tuple) -> Self::Output {
        assert!(self.rows == 4 && self.cols == 4, "Can only multiply 4x4 matrix with tuple");

        let mut ret = rhs.clone();

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
    use std::f32::consts::*;

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
        let _ = &m * &m;
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

        assert_eq!(m, &m * Matrix::identity());
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

    #[test]
    #[should_panic]
    fn test_invert_uninvertable_matrix() {
        let matrix = Matrix::new4x4(-4.0, 2.0, -2.0, -3.0,
                                    9.0, 6.0, 2.0, 6.0,
                                    0.0, -5.0, 1.0, -5.0,
                                    0.0, 0.0, 0.0, 0.0);

        assert_eq!(matrix.determinant(), 0.0);
        matrix.invert();
    }

    #[test]
    fn test_invert() {
        let matrix = Matrix::new4x4(-5.0, 2.0, 6.0, -8.0,
                                    1.0, -5.0, 1.0, 8.0,
                                    7.0, 7.0, -6.0, -7.0,
                                    1.0, -3.0, 7.0, 4.0);

        let expected = Matrix::new4x4(0.21804512, 0.45112783, 0.24060151, -0.04511278,
                                      -0.8082707, -1.456767, -0.44360903, 0.5206767,
                                      -0.078947365, -0.2236842, -0.05263158, 0.19736843,
                                      -0.52255636, -0.81390977, -0.30075186, 0.30639097);

        let inverted = matrix.invert();

        assert_eq!(matrix.determinant(), 532.0);
        assert_eq!(matrix.cofactor(2, 3), -160.0);
        assert_eq!(inverted.at(3, 2), -160.0 / 532.0);
        assert_eq!(matrix.cofactor(3, 2), 105.0);
        assert_eq!(inverted.at(2, 3), 105.0 / 532.0);
        assert_eq!(inverted, expected);
    }

    #[test]
    fn can_multiple_product_by_inverse() {
        let a = Matrix::new4x4(3.0, -9.0, 7.0, 3.0,
                               3.0, -8.0, 2.0, -9.0,
                               -4.0, 4.0, 4.0, 1.0,
                               -6.0, 5.0, -1.0, 1.0);

        let b = Matrix::new4x4(8.0, 2.0, 2.0, 2.0,
                               3.0, -1.0, 7.0, 0.0,
                               7.0, 0.0, 5.0, 4.0,
                               6.0, -2.0, 0.0, 5.0);

        let c = &a * &b;

        assert!(approx_equal(c * b.invert(), a));
    }

    #[test]
    fn translating_at_point() {
        let transformation = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(transformation * p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn translating_at_point_inverse() {
        let transformation = Matrix::translation(5.0, -3.0, 2.0);
        let inv = transformation.invert();
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transformation = Matrix::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(transformation * v, v);
    }

    #[test]
    fn scaling_a_point() {
        let transformation = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(transformation * p, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_a_vector() {
        let transformation = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(transformation * v, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_a_vector_inverse() {
        let transformation = Matrix::scaling(2.0, 3.0, 4.0);
        let inv = transformation.invert();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn negative_scaling_is_reflection() {
        let transformation = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transformation * p, Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let full_quarter = Matrix::rotation_x(PI / 2.0);
        let p = Tuple::point(0.0, 1.0, 0.0);

        assert_eq!(half_quarter * p, Tuple::point(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0));
        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn rotating_point_around_x_axis_inverse() {
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let inv = half_quarter.invert();
        let p = Tuple::point(0.0, 1.0, 0.0);

        assert_eq!(inv * p, Tuple::point(0.0, SQRT_2 / 2.0, - SQRT_2 / 2.0));
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let full_quarter = Matrix::rotation_y(PI / 2.0);
        let p = Tuple::point(0.0, 0.0, 1.0);

        assert_eq!(half_quarter * p, Tuple::point(SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0));
        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let full_quarter = Matrix::rotation_z(PI / 2.0);
        let p = Tuple::point(0.0, 1.0, 0.0);

        assert_eq!(half_quarter * p, Tuple::point(- SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0));
        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_point_x_in_proportion_to_y() {
        let transformation = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transformation * p, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_point_x_in_proportion_to_z() {
        let transformation = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transformation * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_point_y_in_proportion_to_x() {
        let transformation = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transformation * p, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_point_y_in_proportion_to_z() {
        let transformation = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transformation * p, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_point_z_in_proportion_to_x() {
        let transformation = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transformation * p, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_point_z_in_proportion_to_y() {
        let transformation = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transformation * p, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let rotation = Matrix::rotation_x(FRAC_PI_2);
        let scaling = Matrix::scaling(5.0, 5.0, 5.0);
        let translation = Matrix::translation(10.0, 5.0, 7.0);

        let p2 = &rotation * p;
        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));

        let p3 = &scaling * p2;
        assert_eq!(p3, Tuple::point(5.0, -5.0, -0.0000002));

        let p4 = &translation * p3;
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));

        let transformation = &translation * &scaling * &rotation;
        assert_eq!(transformation * p, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn transformations_fluent_api() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let transformation =
            Matrix::identity()
            .rotate_x(FRAC_PI_2)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);

        assert_eq!(transformation * p, Tuple::point(15.0, 0.0, 7.0));
    }

    fn approx_equal(a: Matrix, b: Matrix) -> bool {
        for i in 0..a.data.len() {
            if (a.data[i] - b.data[i]).abs() > 0.001 {
                return false;
            }
        }
        true
    }
}
