use crate::math;
use crate::matrix::{Matrix, rotation_x, rotation_y, rotation_z, scaling, shearing, translation, view_transformation};
use crate::tuples::{points, vectors};
use crate::tuples::points::Point;

#[test]
fn test_translation() {
    let transform = translation(5.0, -3.0, 2.0);
    let point = points::new(-3.0, 4.0, 5.0);
    assert_eq!(transform * point, points::new(2.0, 1.0, 7.0));
}

/// Tests multiplying by the inverse of a translation matrix
#[test]
fn test_translation_inverse() {
    let maybe_inv = translation(5.0, -3.0, 2.0).inverse();
    let point = points::new(-3.0, 4.0, 5.0);
    match maybe_inv {
        Some(transform) => assert_eq!(transform * point, points::new(-8.0, 7.0, 3.0)),
        None => panic!("No inverse")
    }
}

/// Tests that translation does not affect vectors
#[test]
fn test_translation_with_vectors() {
    let transform = translation(5.0, -3.0, 2.0);
    let vector = vectors::new(-3.0, 4.0, 5.0);
    assert_eq!(transform * vector, vector);
}

#[test]
fn test_scaling_with_a_point() {
    let transform = scaling(2.0, 3.0, 4.0);
    let point = points::new(-4.0, 6.0, 8.0);
    assert_eq!(transform * point, points::new(-8.0, 18.0, 32.0));
}

#[test]
fn test_scaling_with_a_vector() {
    let transform = scaling(2.0, 3.0, 4.0);
    let vec = vectors::new(-4.0, 6.0, 8.0);
    assert_eq!(transform * vec, vectors::new(-8.0, 18.0, 32.0));
}

#[test]
fn test_inverse_scaling() {
    let transform = scaling(2.0, 3.0, 4.0);
    match transform.inverse() {
        Some(inv) => {
            let vec = vectors::new(-4.0, 6.0, 8.0);
            assert_eq!(inv * vec, vectors::new(-2.0, 2.0, 2.0));
        }
        None => panic!("No inverse")
    }
}

/// Tests reflection. Reflection in this case is just scaling by
/// a negative value.
#[test]
fn test_reflection() {
    let transform = scaling(-1.0, 1.0, 1.0);
    let point = points::new(2.0, 3.0, 4.0);
    assert_eq!(transform * point, points::new(-2.0, 3.0, 4.0));
}

#[test]
fn test_rotation_around_x() {
    let point = points::new(0.0, 1.0, 0.0);
    let half_quarter = rotation_x(math::PI / 4.0);
    let full_quarter = rotation_x(math::PI / 2.0);
    assert_eq!(half_quarter * &point, points::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0));
    assert_eq!(full_quarter * point, points::new(0.0, 0.0, 1.0));
}

/// Tests the inverse of the x-rotation. It should rotate in
/// the opposite direction
#[test]
fn test_inverse_x_rotation() {
    let point = points::new(0.0, 1.0, 0.0);
    let half_quarter = rotation_x(math::PI / 4.0);
    match half_quarter.inverse() {
        Some(inv) =>
            assert_eq!(inv * point, points::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0)),
        None => panic!("No inverse")
    }
}

#[test]
fn test_rotation_around_y() {
    let point = points::new(0.0, 0.0, 1.0);
    let half_quarter = rotation_y(math::PI / 4.0);
    let full_quarter = rotation_y(math::PI / 2.0);
    assert_eq!(half_quarter * &point, points::new(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0));
    assert_eq!(full_quarter * point, points::new(1.0, 0.0, 0.0));
}

#[test]
fn test_rotation_around_z() {
    let point = points::new(0.0, 1.0, 0.0);
    let half_quarter = rotation_z(math::PI / 4.0);
    let full_quarter = rotation_z(math::PI / 2.0);
    assert_eq!(half_quarter * &point, points::new(-2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0));
    assert_eq!(full_quarter * point, points::new(-1.0, 0.0, 0.0));
}

#[test]
fn test_shearing() {
    let point = points::new(2.0, 3.0, 4.0);

    let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(transform * &point, points::new(5.0, 3.0, 4.0));

    let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(transform * &point, points::new(6.0, 3.0, 4.0));

    let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    assert_eq!(transform * &point, points::new(2.0, 5.0, 4.0));

    let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    assert_eq!(transform * &point, points::new(2.0, 7.0, 4.0));

    let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    assert_eq!(transform * &point, points::new(2.0, 3.0, 6.0));

    let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    assert_eq!(transform * &point, points::new(2.0, 3.0, 7.0));
}

/// Tests to see if individual transformations are applied in sequence.
#[test]
fn test_transformations_sequence() {
    let point = points::new(1.0, 0.0, 1.0);
    let a = rotation_x(math::PI / 2.0);
    let b = scaling(5.0, 5.0, 5.0);
    let c = translation(10.0, 5.0, 7.0);

    let point = a * point;  // apply rotation
    assert_eq!(point, points::new(1.0, -1.0, 0.0));

    let point = b * point;  // then scale
    assert_eq!(point, points::new(5.0, -5.0, 0.0));

    let point = c * point;  // then translate
    assert_eq!(point, points::new(15.0, 0.0, 7.0));
}

#[test]
fn test_transformations_reverse_order() {
    let point = points::new(1.0, 0.0, 1.0);
    let a = rotation_x(math::PI / 2.0);
    let b = scaling(5.0, 5.0, 5.0);
    let c = translation(10.0, 5.0, 7.0);
    let transformations = c * b * a;
    assert_eq!(transformations * point, points::new(15.0, 0.0, 7.0));
}

/// Tests the transformation matrix for the default orientation
#[test]
fn test_view_default_orientation() {
    let from = Point::origin();
    let to = points::new(0.0, 0.0, -1.0);
    let up = vectors::new(0.0, 1.0, 0.0);
    let t = view_transformation(from, to, up);
    assert_eq!(t, Matrix::id44());
}

/// Tests a view transformation matrix looking in positive z direction
#[test]
fn test_view_positive_z_direction() {
    let from = Point::origin();
    let to = points::new(0.0, 0.0, 1.0);
    let up = vectors::new(0.0, 1.0, 0.0);
    let t = view_transformation(from, to, up);
    assert_eq!(t, scaling(-1.0, 1.0, -1.0));
}

#[test]
fn test_view_transformation_moves_the_world() {
    let from = points::new(0.0, 0.0, 8.0);
    let to = Point::origin();
    let up = vectors::new(0.0, 1.0, 0.0);
    let t = view_transformation(from, to, up);
    assert_eq!(t, translation(0.0, 0.0, -8.0));
}

#[test]
fn test_an_arbitrary_view_transformation() {
    let from = points::new(1.0, 3.0, 2.0);
    let to = points::new(4.0, -2.0, 8.0);
    let up = vectors::new(1.0, 1.0, 0.0);
    let t = view_transformation(from, to, up);
    assert_eq!(t.round_items(5), Matrix::new44(&[
        -0.50709, 0.50709, 0.67612, -2.36643,
        0.76772, 0.60609, 0.12122, -2.82843,
        -0.35857, 0.59761, -0.71714, 0.00000,
        0.00000, 0.00000, 0.00000, 1.00000,
    ]));
}