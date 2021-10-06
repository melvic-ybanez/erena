use crate::tuples::{Tuple, Vector, Color, Point};

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
    let point = Point::new(4.0, -4.0, 3.0);
    assert_eq!(point.0, Tuple::new(4.0, -4.0, 3.0, 1.0));
}

#[test]
fn test_vector_creation() {
    let vector = Vector::new(4.0, -4.0, 3.0);
    assert_eq!(vector.0, Tuple::new(4.0, -4.0, 3.0, 0.0));
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
    let point1 = Point::new(3.0, 2.0, 1.0);
    let point2 = Point::new(5.0, 6.0, 7.0);
    assert_eq!(point1.0 - point2.0, Vector::new(-2.0, -4.0, -6.0).0);
}

/// Tests subtracting a vector from a point
#[test]
fn test_point_vector_subtraction() {
    let point = Point::new(3.0, 2.0, 1.0);
    let vector = Vector::new(5.0, 6.0, 7.0);
    assert_eq!(point.0 - vector.0, Point::new(-2.0, -4.0, -6.0).0);
}

/// Subtracting two vectors
#[test]
fn test_vectors_subtraction() {
    let Vector(vec1) = Vector::new(3.0, 2.0, 1.0);
    let Vector(vec2) = Vector::new(5.0, 6.0, 7.0);
    assert_eq!(vec1 - vec2, Vector::new(-2.0, -4.0, -6.0).0);
}

/// Subtracting a vector from the zero vector
#[test]
fn test_zero_vector_subtraction() {
    let Vector(zero) = Vector::zero();
    let Vector(vec) = Vector::new(1.0, -2.0, 3.0);
    assert_eq!(zero - vec, Vector::new(-1.0, 2.0, -3.0).0);
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
    assert_eq!(Vector::new(1.0, 0.0, 0.0).0.magnitude(), 1.0);
    assert_eq!(Vector::new(0.0, 1.0, 0.0).0.magnitude(), 1.0);
    assert_eq!(Vector::new(0.0, 0.0, 1.0).0.magnitude(), 1.0);
    assert_eq!(Vector::new(1.0, 2.0, 3.0).0.magnitude(), (14 as f64).sqrt());
    assert_eq!(Vector::new(-1.0, -2.0, -3.0).0.magnitude(), (14 as f64).sqrt());
}

#[test]
fn test_normalization() {
    assert_eq!(Vector::new(4.0, 0.0, 0.0).normalize(), Vector::new(1.0, 0.0, 0.0));
    let sqrt14 = (14 as f64).sqrt();
    assert_eq!(Vector::new(1.0, 2.0, 3.0).normalize(),
               Vector::new(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14));
    // magnitude of a normalize vector
    assert_eq!(Vector::new(1.0, 2.0, 3.0).normalize().0.magnitude(), 1.0);
}

#[test]
fn test_dot_product() {
    let Vector(a) = Vector::new(1.0, 2.0, 3.0);
    let Vector(b) = Vector::new(2.0, 3.0, 4.0);
    assert_eq!(a.dot(b), 20.0);
}

#[test]
fn test_cross_product() {
    let a = Vector::new(1.0, 2.0, 3.0);
    let b = Vector::new(2.0, 3.0, 4.0);
    assert_eq!(a.cross(b), Vector::new(-1.0, 2.0, -1.0));
    assert_eq!(b.cross(a), Vector::new(1.0, -2.0, 1.0));
}

/// Colors are rgb tuples
#[test]
fn test_colors_as_tuples() {
    let color = Color::new(-0.5, 0.4, 1.7);
    assert_eq!(color.red(), -0.5);
    assert_eq!(color.green(), 0.4);
    assert_eq!(color.blue(), 1.7);
}

#[test]
fn test_colors_addition() {
    let Color(c1) = Color::new(0.9, 0.6, 0.75);
    let Color(c2) = Color::new(0.7, 0.1, 0.25);
    let Color(sum) = Color::new(1.6, 0.7, 1.0);
    assert_eq!(c1 + c2, sum);
}

#[test]
fn test_colors_subtraction() {
    let Color(c1) = Color::new(0.9, 0.6, 0.75);
    let Color(c2) = Color::new(0.7, 0.1, 0.25);
    let Color(result) = Color::new(0.2, 0.5, 0.5);
    assert_eq!(c1 - c2, result);
}

#[test]
fn test_colors_scalar_multiplication() {
    let Color(c) = Color::new(0.2, 0.3, 0.4);
    assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8).0);
}

#[test]
fn test_hadamard_product() {
    let c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);
    assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
}