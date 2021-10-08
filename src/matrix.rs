use crate::math::Real;
use std::ops::Index;
use crate::math;

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

    fn new44(elements: &[Real; 16]) -> Matrix {
        Matrix::new(elements.to_vec().chunks(4).map(|x| x.to_vec()).collect())
    }
}

impl Index<math::Idx> for Matrix {
    type Output = Real;

    fn index(&self, index: math::Idx) -> &Self::Output {
        let (row, col) = index;
        &self.elements[math::index_of(col, row, self.width)]
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
}