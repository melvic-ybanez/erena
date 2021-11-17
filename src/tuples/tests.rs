use crate::tuples;
use crate::tuples::vectors::Vector;
use crate::tuples::{colors, points, vectors};

/// A tuple with w=1.0 is a point
#[test]
fn test_point_w() {
    let tuple = tuples::new(4.3, -4.2, 3.1, 1.0);
    assert_eq!(tuple.x, 4.3);
    assert_eq!(tuple.y, -4.2);
    assert_eq!(tuple.z, 3.1);
    assert_eq!(tuple.w, 1.0);
    assert!(tuple.is_point());
}

/// A tuple with w=0 is a vector
#[test]
fn test_vector_w() {
    let tuple = tuples::new(4.3, -4.2, 3.1, 0.0);
    assert_eq!(tuple.x, 4.3);
    assert_eq!(tuple.y, -4.2);
    assert_eq!(tuple.z, 3.1);
    assert_eq!(tuple.w, 0.0);
    assert!(tuple.is_vector());
}

#[test]
fn test_point_creation() {
    let point = points::new(4.0, -4.0, 3.0);
    assert_eq!(point.to_tuple(), tuples::new(4.0, -4.0, 3.0, 1.0));
}

#[test]
fn test_vector_creation() {
    let vector = vectors::new(4.0, -4.0, 3.0);
    assert_eq!(vector.to_tuple(), tuples::new(4.0, -4.0, 3.0, 0.0));
}

#[test]
fn test_tuples_addition() {
    let a = tuples::new(3.0, -2.0, 5.0, 1.0);
    let b = tuples::new(-2.0, 3.0, 1.0, 0.0);
    assert_eq!(a + b, tuples::new(1.0, 1.0, 6.0, 1.0));
}

/// Subtracting two points
#[test]
fn test_points_subtraction() {
    let point1 = points::new(3.0, 2.0, 1.0);
    let point2 = points::new(5.0, 6.0, 7.0);
    assert_eq!(
        (point1 - point2).to_vector(),
        vectors::new(-2.0, -4.0, -6.0)
    );
}

/// Tests subtracting a vector from a point
#[test]
fn test_point_vector_subtraction() {
    let point = points::new(3.0, 2.0, 1.0);
    let vector = vectors::new(5.0, 6.0, 7.0);
    assert_eq!(point - vector, points::new(-2.0, -4.0, -6.0));
}

/// Subtracting two vectors
#[test]
fn test_vectors_subtraction() {
    let vec1 = vectors::new(3.0, 2.0, 1.0);
    let vec2 = vectors::new(5.0, 6.0, 7.0);
    assert_eq!(vec1 - vec2, vectors::new(-2.0, -4.0, -6.0));
}

/// Subtracting a vector from the zero vector
#[test]
fn test_zero_vector_subtraction() {
    let zero = Vector::zero();
    let vec = vectors::new(1.0, -2.0, 3.0);
    assert_eq!(zero - vec, vectors::new(-1.0, 2.0, -3.0));
}

#[test]
fn test_tuple_negation() {
    let tuple = tuples::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(-tuple, tuples::new(-1.0, 2.0, -3.0, 4.0));
}

#[test]
fn test_scalar_multiplication() {
    let tuple = tuples::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(tuple * 3.5, tuples::new(3.5, -7.0, 10.5, -14.0));
}

#[test]
fn test_multiply_by_fraction() {
    let tuple = tuples::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(tuple * 0.5, tuples::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn test_scalar_division() {
    let tuple = tuples::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(tuple / 2.0, tuples::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn test_magnitude() {
    assert_eq!(vectors::new(1.0, 0.0, 0.0).magnitude(), 1.0);
    assert_eq!(vectors::new(0.0, 1.0, 0.0).magnitude(), 1.0);
    assert_eq!(vectors::new(0.0, 0.0, 1.0).magnitude(), 1.0);
    assert_eq!(vectors::new(1.0, 2.0, 3.0).magnitude(), (14 as f64).sqrt());
    assert_eq!(
        vectors::new(-1.0, -2.0, -3.0).magnitude(),
        (14 as f64).sqrt()
    );
}

#[test]
fn test_normalization() {
    assert_eq!(
        vectors::new(4.0, 0.0, 0.0).normalize(),
        vectors::new(1.0, 0.0, 0.0)
    );
    let sqrt14 = (14 as f64).sqrt();
    assert_eq!(
        vectors::new(1.0, 2.0, 3.0).normalize(),
        vectors::new(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14)
    );
    // magnitude of a normalize vector
    assert_eq!(vectors::new(1.0, 2.0, 3.0).normalize().magnitude(), 1.0);
}

#[test]
fn test_dot_product() {
    let a = vectors::new(1.0, 2.0, 3.0);
    let b = vectors::new(2.0, 3.0, 4.0);
    assert_eq!(a.dot(b), 20.0);
}

#[test]
fn test_cross_product() {
    let a = vectors::new(1.0, 2.0, 3.0);
    let b = vectors::new(2.0, 3.0, 4.0);
    assert_eq!(a.cross(b), vectors::new(-1.0, 2.0, -1.0));
    assert_eq!(b.cross(a), vectors::new(1.0, -2.0, 1.0));
}

/// Colors are rgb tuples
#[test]
fn test_colors_as_tuples() {
    let color = colors::new(-0.5, 0.4, 1.7);
    assert_eq!(color.red_value(), -0.5);
    assert_eq!(color.green_value(), 0.4);
    assert_eq!(color.blue_value(), 1.7);
}

#[test]
fn test_colors_addition() {
    let c1 = colors::new(0.9, 0.6, 0.75);
    let c2 = colors::new(0.7, 0.1, 0.25);
    let sum = colors::new(1.6, 0.7, 1.0);
    assert_eq!(c1 + c2, sum);
}

#[test]
fn test_colors_subtraction() {
    let c1 = colors::new(0.9, 0.6, 0.75);
    let c2 = colors::new(0.7, 0.1, 0.25);
    let result = colors::new(0.2, 0.5, 0.5);
    assert_eq!(c1 - c2, result);
}

#[test]
fn test_colors_scalar_multiplication() {
    let c = colors::new(0.2, 0.3, 0.4);
    assert_eq!(c * 2.0, colors::new(0.4, 0.6, 0.8));
}

#[test]
fn test_hadamard_product() {
    let c1 = colors::new(1.0, 0.2, 0.4);
    let c2 = colors::new(0.9, 1.0, 0.1);
    assert_eq!(c1 * c2, colors::new(0.9, 0.2, 0.04));
}

#[test]
fn test_reflecting_a_vector_approaching_at_45() {
    let vector = vectors::new(1.0, -1.0, 0.0);
    let n = vectors::new(0.0, 1.0, 0.0);
    let r = vector.reflect(n);
    assert_eq!(r, vectors::new(1.0, 1.0, 0.0));
}

#[test]
fn test_reflecting_a_vector_off_a_slanted_surface() {
    let vector = vectors::new(0.0, -1.0, 0.0);
    let n = vectors::new(2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0);
    let r = vector.reflect(n);
    assert_eq!(r, vectors::new(1.0, 0.0, 0.0));
}
