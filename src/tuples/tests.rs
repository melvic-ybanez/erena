use crate::tuples::{Tuple, Vector};

/// A tuple with w=1.0 is a point
#[test]
fn test_point_w() {
    let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
    assert_eq!(tuple.x, 4.3);
    assert_eq!(tuple.y, -4.2);
    assert_eq!(tuple.z, 3.1);
    assert_eq!(tuple.w, 1.0);
    assert!(tuple.is_point());
}

/// A tuple with w=0 is a vector
#[test]
fn test_vector_w() {
    let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
    assert_eq!(tuple.x, 4.3);
    assert_eq!(tuple.y, -4.2);
    assert_eq!(tuple.z, 3.1);
    assert_eq!(tuple.w, 0.0);
    assert!(tuple.is_vector());
}

#[test]
fn test_point_creation() {
    let point = Tuple::point(4.0, -4.0, 3.0);
    assert_eq!(point, Tuple::new(4.0, -4.0, 3.0, 1.0));
}

#[test]
fn test_vector_creation() {
    let vector = Tuple::vector(4.0, -4.0, 3.0);
    assert_eq!(vector, Tuple::new(4.0, -4.0, 3.0, 0.0));
}

#[test]
fn test_tuples_addition() {
    let a = Tuple::new(3.0, -2.0, 5.0, 1.0);
    let b = Tuple::new(-2.0, 3.0, 1.0, 0.0);
    assert_eq!(a + b, Tuple::new(1.0, 1.0, 6.0, 1.0));
}

/// Subtracting two points
#[test]
fn test_points_subtraction() {
    let point1 = Tuple::point(3.0, 2.0, 1.0);
    let point2 = Tuple::point(5.0, 6.0, 7.0);
    assert_eq!(point1 - point2, Tuple::vector(-2.0, -4.0, -6.0));
}

/// Tests subtracting a vector from a point
#[test]
fn test_point_vector_subtraction() {
    let point = Tuple::point(3.0, 2.0, 1.0);
    let vector = Tuple::vector(5.0, 6.0, 7.0);
    assert_eq!(point - vector, Tuple::point(-2.0, -4.0, -6.0));
}

/// Subtracting two vectors
#[test]
fn test_vectors_subtraction() {
    let vec1 = Tuple::vector(3.0, 2.0, 1.0);
    let vec2 = Tuple::vector(5.0, 6.0, 7.0);
    assert_eq!(vec1 - vec2, Tuple::vector(-2.0, -4.0, -6.0));
}

/// Subtracting a vector from the zero vector
#[test]
fn test_zero_vector_subtraction() {
    let zero = Vector::zero();
    let vec = Tuple::vector(1.0, -2.0, 3.0);
    assert_eq!(zero - vec, Tuple::vector(-1.0, 2.0, -3.0));
}

#[test]
fn test_tuple_negation() {
    let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(-tuple, Tuple::new(-1.0, 2.0, -3.0, 4.0));
}

#[test]
fn test_scalar_multiplication() {
    let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(tuple * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
}

#[test]
fn test_multiply_by_fraction() {
    let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(tuple * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn test_scalar_division() {
    let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(tuple / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn test_magnitude() {
    assert_eq!(Tuple::vector(1.0, 0.0, 0.0).magnitude(), 1.0);
    assert_eq!(Tuple::vector(0.0, 1.0, 0.0).magnitude(), 1.0);
    assert_eq!(Tuple::vector(0.0, 0.0, 1.0).magnitude(), 1.0);
    assert_eq!(Tuple::vector(1.0, 2.0, 3.0).magnitude(), (14 as f64).sqrt());
    assert_eq!(Tuple::vector(-1.0, -2.0, -3.0).magnitude(), (14 as f64).sqrt());
}

#[test]
fn test_normalization() {
    assert_eq!(Tuple::vector(4.0, 0.0, 0.0).normalize(), Tuple::vector(1.0, 0.0, 0.0));
    let sqrt14 = (14 as f64).sqrt();
    assert_eq!(Tuple::vector(1.0, 2.0, 3.0).normalize(),
               Tuple::vector(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14));
    // magnitude of a normalize vector
    assert_eq!(Tuple::vector(1.0, 2.0, 3.0).normalize().magnitude(), 1.0);
}

#[test]
fn test_dot_product() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    assert_eq!(a.dot(b), 20.0);
}