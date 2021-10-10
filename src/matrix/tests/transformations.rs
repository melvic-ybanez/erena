use crate::matrix::{translation, scaling, rotation_x, rotation_y, rotation_z, shearing};
use crate::tuples::{Point, Vector};
use crate::math;

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

#[test]
fn test_scaling_with_a_point() {
    let transform = scaling(2.0, 3.0, 4.0);
    let point = Point::new(-4.0, 6.0, 8.0);
    assert_eq!(transform * point, Point::new(-8.0, 18.0, 32.0));
}

#[test]
fn test_scaling_with_a_vector() {
    let transform = scaling(2.0, 3.0, 4.0);
    let vec = Vector::new(-4.0, 6.0, 8.0);
    assert_eq!(transform * vec, Vector::new(-8.0, 18.0, 32.0));
}

#[test]
fn test_inverse_scaling() {
    let transform = scaling(2.0, 3.0, 4.0);
    match transform.inverse() {
        Some(inv) => {
            let vec = Vector::new(-4.0, 6.0, 8.0);
            assert_eq!(inv * vec, Vector::new(-2.0, 2.0, 2.0));
        },
        None => assert!(false)
    }
}

/// Tests reflection. Reflection in this case is just scaling by
/// a negative value.
#[test]
fn test_reflection() {
    let transform = scaling(-1.0, 1.0, 1.0);
    let point = Point::new(2.0, 3.0, 4.0);
    assert_eq!(transform * point, Point::new(-2.0, 3.0, 4.0));
}

#[test]
fn test_rotation_around_x() {
    let point = Point::new(0.0, 1.0, 0.0);
    let half_quarter = rotation_x(math::PI / 4.0);
    let full_quarter = rotation_x(math::PI / 2.0);
    assert_eq!(half_quarter * &point, Point::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0));
    assert_eq!(full_quarter * point, Point::new(0.0, 0.0, 1.0));
}

/// Tests the inverse of the x-rotation. It should rotate in
/// the opposite direction
#[test]
fn test_inverse_x_rotation() {
    let point = Point::new(0.0, 1.0, 0.0);
    let half_quarter = rotation_x(math::PI / 4.0);
    match half_quarter.inverse() {
        Some(inv) =>
            assert_eq!(inv * point, Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0)),
        None => assert!(false)
    }
}

#[test]
fn test_rotation_around_y() {
    let point = Point::new(0.0, 0.0, 1.0);
    let half_quarter = rotation_y(math::PI / 4.0);
    let full_quarter = rotation_y(math::PI / 2.0);
    assert_eq!(half_quarter * &point, Point::new(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0));
    assert_eq!(full_quarter * point, Point::new(1.0, 0.0, 0.0));
}

#[test]
fn test_rotation_around_z() {
    let point = Point::new(0.0, 1.0, 0.0);
    let half_quarter = rotation_z(math::PI / 4.0);
    let full_quarter = rotation_z(math::PI / 2.0);
    assert_eq!(half_quarter * &point, Point::new(-2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0));
    assert_eq!(full_quarter * point, Point::new(-1.0, 0.0, 0.0));
}

#[test]
fn test_shearing() {
    let point = Point::new(2.0, 3.0, 4.0);

    let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(transform * &point, Point::new(5.0, 3.0, 4.0));

    let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(transform * &point, Point::new(6.0, 3.0, 4.0));

    let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    assert_eq!(transform * &point, Point::new(2.0, 5.0, 4.0));

    let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    assert_eq!(transform * &point, Point::new(2.0, 7.0, 4.0));

    let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    assert_eq!(transform * &point, Point::new(2.0, 3.0, 6.0));

    let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    assert_eq!(transform * &point, Point::new(2.0, 3.0, 7.0));
}