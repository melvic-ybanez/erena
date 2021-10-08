use crate::math::Real;
use std::ops::Index;
use crate::math;

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

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

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
}