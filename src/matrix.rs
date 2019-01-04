use std::ops::Mul;

use crate::tuple::Tuple;

/// Square Matrix
#[derive(Debug)]
pub struct Matrix {
    pub dim: usize,
    elems: Vec<f64>,
}

impl Matrix {
    pub fn from_vector(d: usize, e: &[f64]) -> Matrix {
        Matrix {
            dim: d,
            elems: e.to_vec(),
        }
    }

    pub fn identity() -> Matrix {
        let v = [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        Matrix::from_vector(4, &v)
    }

    pub fn at(&self, r: usize, c: usize) -> f64 {
        self.elems[r * self.dim + c]
    }

    pub fn tuple_prod(&self, t: Tuple) -> Tuple {
        assert!(self.dim == 4);
        let dot = |r: usize| -> f64 {
            self.at(r, 0) * t.0 + self.at(r, 1) * t.1 + self.at(r, 2) * t.2 + self.at(r, 3) * t.3
        };
        Tuple(dot(0), dot(1), dot(2), dot(3))
    }

    pub fn transpose(&self) -> Matrix {
        let mut v: Vec<f64> = Vec::with_capacity(self.dim * self.dim);
        for r in 0..self.dim {
            for c in 0..self.dim {
                v.push(self.at(c, r));
            }
        }
        Matrix::from_vector(self.dim, &v)
    }

    pub fn det(&self) -> f64 {
        if self.dim == 2 {
            return self.at(0, 0) * self.at(1, 1) - self.at(0, 1) * self.at(1, 0);
        } else {
            (0..self.dim)
                .map(|c| self.at(0, c) * self.cofactor(0, c))
                .sum()
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut v: Vec<f64> = Vec::with_capacity((self.dim - 1) * (self.dim - 1));
        for r in 0..self.dim {
            for c in 0..self.dim {
                if r != row && c != col {
                    v.push(self.at(r, c));
                }
            }
        }
        Matrix::from_vector(self.dim - 1, &v)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).det()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if row + col % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        fn compare_elems(a: &[f64], b: &[f64]) -> bool {
            for (x, y) in a.iter().zip(b.iter()) {
                if (x - y).abs() >= 1e-6 {
                    return false;
                }
            }
            true
        }

        self.dim == other.dim && compare_elems(&self.elems, &other.elems)
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        assert!(self.dim == other.dim);
        let dot = |r: usize, c: usize| -> f64 {
            (0..self.dim).map(|x| self.at(r, x) * other.at(x, c)).sum()
        };

        let mut v = vec![0.0; self.dim * self.dim];
        for i in 0..self.dim {
            for j in 0..self.dim {
                v[j + i * self.dim] = dot(i, j);
            }
        }

        Matrix::from_vector(self.dim, &v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_and_inspecting_4x4_matrix() {
        let elems = vec![
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        ];
        let m = Matrix::from_vector(4, &elems);

        assert_eq!(1.0, m.at(0, 0));
        assert_eq!(4.0, m.at(0, 3));
        assert_eq!(5.5, m.at(1, 0));
        assert_eq!(7.5, m.at(1, 2));
        assert_eq!(11.0, m.at(2, 2));
        assert_eq!(13.5, m.at(3, 0));
        assert_eq!(15.5, m.at(3, 2));
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let elems = vec![-3.0, 5.0, 1.0, -2.0];
        let m = Matrix::from_vector(2, &elems);

        assert_eq!(-3.0, m.at(0, 0));
        assert_eq!(5.0, m.at(0, 1));
        assert_eq!(1.0, m.at(1, 0));
        assert_eq!(-2.0, m.at(1, 1));
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let elems = vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0];
        let m = Matrix::from_vector(3, &elems);

        assert_eq!(-3.0, m.at(0, 0));
        assert_eq!(-2.0, m.at(1, 1));
        assert_eq!(1.0, m.at(2, 2));
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let v = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ];
        let m1 = Matrix::from_vector(4, &v);
        let m2 = Matrix::from_vector(4, &v);

        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let v1 = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ];
        let v2 = vec![
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        ];

        let m1 = Matrix::from_vector(4, &v1);
        let m2 = Matrix::from_vector(4, &v2);

        assert_ne!(m1, m2);
    }

    #[test]
    fn multiplying_two_matrices() {
        let v1 = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ];
        let v2 = vec![
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        ];
        let m1 = Matrix::from_vector(4, &v1);
        let m2 = Matrix::from_vector(4, &v2);

        let v3 = vec![
            20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0,
            46.0, 42.0,
        ];
        let m3 = Matrix::from_vector(4, &v3);

        assert_eq!(m3, m1 * m2);
    }

    #[test]
    fn matrix_multiplied_by_tuple() {
        let v = vec![
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ];
        let m = Matrix::from_vector(4, &v);
        let t = Tuple(1.0, 2.0, 3.0, 1.0);

        assert_eq!(Tuple(18.0, 24.0, 33.0, 1.0), m.tuple_prod(t));
    }

    #[test]
    fn multiplying_matrix_by_identity_matrix() {
        let v = vec![
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
        ];
        let m = Matrix::from_vector(4, &v);
        let m2 = Matrix::from_vector(4, &v); // TODO figure out how to deal with borrow checker
        let p = m2 * Matrix::identity();

        assert_eq!(m, p);
    }

    #[test]
    fn transposing_matrix() {
        let v1 = vec![
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        ];
        let m = Matrix::from_vector(4, &v1);

        let v2 = vec![
            0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
        ];
        let t = Matrix::from_vector(4, &v2);

        assert_eq!(t, m.transpose());
    }

    #[test]
    fn transposing_identity_matrix() {
        let i = Matrix::identity();

        assert_eq!(Matrix::identity(), i.transpose());
    }

    #[test]
    fn calculating_determinant_of_2x2_matrix() {
        let v = vec![1.0, 5.0, -3.0, 2.0];
        let m = Matrix::from_vector(2, &v);

        assert_eq!(17.0, m.det());
    }

    #[test]
    fn submatrix_of_3x3_matrix_is_2x2_matrix() {
        let v1 = vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0];
        let m1 = Matrix::from_vector(3, &v1);

        let v2 = vec![-3.0, 2.0, 0.0, 6.0];
        let m2 = Matrix::from_vector(2, &v2);

        assert_eq!(m2, m1.submatrix(0, 2));
    }

    #[test]
    fn submatrix_of_4x4_matrix_is_3x3_matrix() {
        let v1 = vec![
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
        ];
        let m1 = Matrix::from_vector(4, &v1);

        let v2 = vec![-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0];
        let m2 = Matrix::from_vector(3, &v2);

        assert_eq!(m2, m1.submatrix(2, 1));
    }

    #[test]
    fn calculating_minor_of_3x3_matrix() {
        let v = vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0];
        let a = Matrix::from_vector(3, &v);
        let b = a.submatrix(1, 0);

        assert_eq!(25.0, b.det());
        assert_eq!(25.0, a.minor(1, 0));
    }

    #[test]
    fn calculating_cofactor_of_3x3_matrix() {
        let v = vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0];
        let a = Matrix::from_vector(3, &v);

        assert_eq!(-12.0, a.minor(0, 0));
        assert_eq!(-12.0, a.cofactor(0, 0));
        assert_eq!(25.0, a.minor(1, 0));
        assert_eq!(-25.0, a.cofactor(1, 0));
    }

    #[test]
    fn calculating_determinant_of_3x3_matrix() {
        let v = vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0];
        let a = Matrix::from_vector(3, &v);

        assert_eq!(56.0, a.cofactor(0, 0));
        assert_eq!(12.0, a.cofactor(0, 1));
        assert_eq!(-46.0, a.cofactor(0, 2));
        assert_eq!(-196.0, a.det());
    }

    #[test]
    fn calculating_determinant_of_4x4_matrix() {
        let v = vec![
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
        ];
        let a = Matrix::from_vector(4, &v);

        assert_eq!(690.0, a.cofactor(0, 0));
        assert_eq!(447.0, a.cofactor(0, 1));
        assert_eq!(210.0, a.cofactor(0, 2));
        assert_eq!(51.0, a.cofactor(0, 3));
        assert_eq!(-4071.0, a.det());
    }
}
