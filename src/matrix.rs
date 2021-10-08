use crate::math::Real;
use std::ops::{Index, Mul, IndexMut};
use crate::math;
use crate::tuples::Tuple;

#[derive(Debug)]
struct Matrix {
    elements: Vec<Real>,
    width: usize,
    height: usize,
}

impl Matrix {
    fn new(elements: Vec<Vec<Real>>) -> Matrix {
        Matrix {
            width: elements[0].len(),
            height: elements.len(),
            elements: elements.into_iter().flatten().collect::<Vec<Real>>()
        }
    }

    fn with_nxn(n: usize) -> Matrix {
        Matrix::new(vec![vec![0.0; n]; n])
    }

    fn new_nxn(n: usize, elements: &[Real]) -> Matrix {
        Matrix::new(elements.to_vec().chunks(n).map(|x| x.to_vec()).collect())
    }

    fn new44(elements: &[Real; 16]) -> Matrix {
        Matrix::new_nxn(4, elements)
    }

    fn new22(elements: &[Real; 4]) -> Matrix {
        Matrix::new_nxn(2, elements)
    }

    fn new33(elements: &[Real; 9]) -> Matrix {
        Matrix::new_nxn(3, elements)
    }
}

impl Index<math::Idx> for Matrix {
    type Output = Real;

    fn index(&self, index: math::Idx) -> &Self::Output {
        let (row, col) = index;
        &self.elements[math::index_of(col, row, self.width)]
    }
}

impl IndexMut<math::Idx> for Matrix {
    fn index_mut(&mut self, index: math::Idx) -> &mut Self::Output {
        let (row, col) = index;
        &mut self.elements[math::index_of(col, row, self.width)]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }
        for j in 0..self.height {
            for i in 0..self.width {
                if !math::compare_reals(self[(j, i)], other[(j, i)]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Self) -> Self::Output {
        let mut matrix = Matrix::with_nxn(self.width);

        // assumes the width and height have the same size
        let size = self.width;

        let cell = |row: &[Real], col_index: usize| -> Real {
            let mut acc = 0 as Real;
            for i in 0..size {
                acc += row[i] * other[(i, col_index)];
            }
            acc
        };

        for r in 0..size {
            for c in 0..size {
                let row = &self.elements[r * size..r * size + size];
                matrix[(r, c)] = cell(&row, c);
            }
        }

        matrix
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut elems = [0.0; Tuple::LEN];

        for r in 0..Tuple::LEN {
            for c in 0..Tuple::LEN {
                elems[r] += self[(r, c)] * rhs[c];
            }
        }

        Tuple::from_array(&elems)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use crate::tuples::Tuple;

    #[test]
    fn test_4x4_matrix_creation() {
        let matrix = Matrix::new44(&[
            1.0, 2.0, 3.0, 4.0,
            5.5, 6.5, 7.5, 8.5,
            9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5,
        ]);
        assert_eq!(matrix[(0, 0)], 1.0);
        assert_eq!(matrix[(0, 3)], 4.0);
        assert_eq!(matrix[(1, 0)], 5.5);
        assert_eq!(matrix[(1, 2)], 7.5);
        assert_eq!(matrix[(2, 2)], 11.0);
        assert_eq!(matrix[(3, 0)], 13.5);
        assert_eq!(matrix[(3, 2)], 15.5);
    }

    #[test]
    fn test_2x2_matrix() {
        let m = Matrix::new22(&[
           -3.0, 5.0,
            1.0, -2.0
        ]);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 0)], 1.0);
        assert_eq!(m[(1, 1)], -2.0);
    }

    #[test]
    fn test_3x3_matrix() {
        let m = Matrix::new33(&[
            -3.0, 5.0, 0.0,
            1.0, -2.0, -7.0,
            0.0, 1.0, 1.0
        ]);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);
    }

    #[test]
    fn test_matrix_equality() {
        let elems = &[
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ];
        let m = Matrix::new44(elems);
        let m1 = Matrix::new44(elems);
        assert_eq!(m, m1);
    }

    #[test]
    fn test_matrix_inequality() {
        let m = Matrix::new44(&[
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ]);
        let m1 = Matrix::new44(&[
            2.0, 3.0, 4.0, 5.0,
            6.0, 7.0, 8.0, 9.0,
            8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0, 1.0
        ]);
        assert_ne!(m, m1);
    }

    #[test]
    fn test_matrix_multiplication() {
        let m1 = Matrix::new44(&[
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ]);
        let m2 = Matrix::new44(&[
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0,
        ]);
        assert_eq!(m1 * m2, Matrix::new44(&[
            20.0, 22.0, 50.0, 48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0,
        ]));
    }

    #[test]
    fn test_matrix_tuple_multiplication() {
        let matrix = Matrix::new44(&[
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0,
        ]);
        let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(matrix * tuple, Tuple::new(18.0, 24.0, 33.0, 1.0));
    }
}