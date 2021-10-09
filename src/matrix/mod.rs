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
    /// Constructs a new mod from a 2D vector. The
    /// width and height properties are computed from the vector itself.
    /// Note: This constructor assumes the vector represents n x n mod
    /// and not an m x n one.
    fn new(elements: Vec<Vec<Real>>) -> Matrix {
        Matrix {
            width: elements[0].len(),
            height: elements.len(),
            elements: elements.into_iter().flatten().collect::<Vec<Real>>(),
        }
    }

    /// Constructs an n x n mod with all the cells initialized to 0.0.
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

    /// Identity for 4 x 4 mod
    #[inline(always)]
    fn id44() -> Matrix {
        Matrix::new44(&[
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ])
    }

    fn transpose(&self) -> Matrix {
        let mut matrix = Matrix::with_nxn(self.width);
        for r in 0..self.width {
            for c in 0..self.width {
                matrix[(r, c)] = self[(c, r)];
            }
        }
        matrix
    }

    fn determinant(&self) -> Real {
        if self.width == 2 && self.height == 2 {
            // The determinant of a 2 x 2 matrix follows the formula `ad - bc`
            return self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)];
        }
        unimplemented!()
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

/// Two matrices are equal if all elements are equal according to
/// the algorithm for comparing floats.
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
mod tests;