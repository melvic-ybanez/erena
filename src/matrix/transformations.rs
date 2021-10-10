use crate::math::Real;
use crate::matrix::Matrix;

fn translation(x: Real, y: Real, z: Real) -> Matrix {
    Matrix::new44(&[
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0,
    ])
}

#[cfg(test)]
mod tests {
    use crate::matrix::transformations::translation;
    use crate::tuples::{Point, Vector};

    #[test]
    fn test_translation() {
        let transform = translation(5.0, -3.0, 2.0);
        let point = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(transform * point, Point::new(2.0, 1.0, 7.0));
    }

    /// Tests multiplying by the inverse of a translation matrix
    #[test]
    fn test_translation_inverse() {
        let maybe_inv = translation(5.0, -3.0, 2.0).inverse();
        let point = Point::new(-3.0, 4.0, 5.0);
        match maybe_inv {
            Some(transform) => assert_eq!(transform * point, Point::new(-8.0, 7.0, 3.0)),
            None => assert!(false)
        }
    }

    /// Tests that translation does not affect vectors
    #[test]
    fn test_translation_with_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let vector = Vector::new(-3.0, 4.0, 5.0);
        assert_eq!(transform * vector, vector);
    }
}