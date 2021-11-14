use crate::tuples::points;
use crate::shapes::Shape;
use crate::shapes::bounds::Bounds;
use crate::math::Real;
use crate::shapes::cylinders::CylLike;
use crate::matrix::{rotation_x, rotation_y, CanTransform};
use crate::math;

#[test]
fn test_creating_empty_box() {
    let bbox = Bounds::empty();
    let min = points::new(Real::INFINITY, Real::INFINITY, Real::INFINITY);
    let max = (-min).to_point();
    assert_eq!(bbox.min, min);
    assert_eq!(bbox.max, max);
}

#[test]
fn test_adding_points_to_box() {
    let bbox = Bounds::empty();
    let p1 = points::new(-5.0, 2.0, 0.0);
    let p2 = points::new(7.0, 0.0, -3.0);
    let bbox = bbox + p1 + p2;
    assert_eq!(bbox.min, points::new(-5.0, 0.0, -3.0));
    assert_eq!(bbox.max, points::new(7.0, 2.0, 0.0));
}

#[test]
fn test_sphere_bounds() {
    let shape = Shape::sphere();
    let bbox = shape.bounds();
    assert_eq!(bbox.min, points::new(-1.0, -1.0, -1.0));
    assert_eq!(bbox.max, points::new(1.0, 1.0, 1.0));
}

#[test]
fn test_plane_bounds() {
    let bbox = Shape::plane().bounds();
    assert_eq!(bbox.min, points::new(-Real::INFINITY, 0.0, -Real::INFINITY));
    assert_eq!(bbox.max, points::new(Real::INFINITY, 0.0, Real::INFINITY));
}

#[test]
fn test_cube_bounds() {
    let bbox = Shape::cube().bounds();
    assert_eq!(bbox.min, points::new(-1.0, -1.0, -1.0));
    assert_eq!(bbox.max, points::new(1.0, 1.0, 1.0));
}

#[test]
fn test_unbounded_cylinder_bounds() {
    let bbox = Shape::cylinder().bounds();
    assert_eq!(bbox.min, points::new(-1.0, -Real::INFINITY, -1.0));
    assert_eq!(bbox.max, points::new(1.0, Real::INFINITY, 1.0));
}

#[test]
fn test_bounded_cylinder_bounds() {
    let bbox = CylLike::cylinder()
        .min(-5.0).max(3.0)
        .to_shape()
        .bounds();
    assert_eq!(bbox.min, points::new(-1.0, -5.0, -1.0));
    assert_eq!(bbox.max, points::new(1.0, 3.0, 1.0));
}

#[test]
fn test_unbounded_cone_bounds() {
    let bbox = Shape::cone().bounds();
    assert_eq!(bbox.min, points::new(-Real::INFINITY, -Real::INFINITY, -Real::INFINITY));
    assert_eq!(bbox.max, points::new(Real::INFINITY, Real::INFINITY, Real::INFINITY));
}

#[test]
fn test_bounded_cone_bounds() {
    let bbox = CylLike::cone()
        .min(-5.0).max(3.0)
        .to_shape()
        .bounds();
    assert_eq!(bbox.min, points::new(-5.0, -5.0, -5.0));
    assert_eq!(bbox.max, points::new(5.0, 3.0, 5.0));
}

#[test]
fn test_triangle_bounding_box() {
    let p1 = points::new(-3.0, 7.0, 2.0);
    let p2 = points::new(6.0, 2.0, -4.0);
    let p3 = points::new(2.0, -1.0, -1.0);

    let shape = Shape::triangle(p1, p2, p3);
    let bounds = shape.bounds();
    assert_eq!(bounds.min, points::new(-3.0, -1.0, -4.0));
    assert_eq!(bounds.max, points::new(6.0, 7.0, 2.0));
}

#[test]
fn test_bound_of_test_shape() {
    let bbox = Shape::test().bounds();
    assert_eq!(bbox.min, points::new(-1.0, -1.0, -1.0));
    assert_eq!(bbox.max, points::new(1.0, 1.0, 1.0));
}

#[test]
fn test_adding_bounding_boxes() {
    let box1 = Bounds::new(points::new(-5.0, -2.0, 0.0), points::new(7.0, 4.0, 4.0));
    let box2 = Bounds::new(points::new(8.0, -7.0, -2.0), points::new(14.0, 2.0, 8.0));
    let bbox = box1 + box2;
    assert_eq!(bbox.min, points::new(-5.0, -7.0, -2.0));
    assert_eq!(bbox.max, points::new(14.0, 4.0, 8.0));
}

/// Tests to see if a box contains a point
#[test]
fn test_box_containing_point() {
    let bbox = Bounds::new(points::new(5.0, -2.0, 0.0), points::new(11.0, 4.0, 7.0));
    let data = [
        (points::new(5.0, -2.0, 0.0), true),
        (points::new(11.0, 4.0, 7.0), true),
        (points::new(8.0, 1.0, 3.0), true),
        (points::new(3.0, 0.0, 3.0), false),
        (points::new(8.0, -4.0, 3.0), false),
        (points::new(8.0, 1.0, -1.0), false),
        (points::new(13.0, 1.0, 3.0), false),
        (points::new(8.0, 5.0, 3.0), false),
        (points::new(8.0, 1.0, 8.0), false)
    ];
    for (point, contains) in data {
        assert_eq!(bbox.contains_point(point), contains);
    }
}

#[test]
fn text_box_containing_box() {
    let bbox = Bounds::new(points::new(5.0, -2.0, 0.0), points::new(11.0, 4.0, 7.0));
    let data = [
        (points::new(5.0, -2.0, 0.0), points::new(11.0, 4.0, 7.0), true),
        (points::new(6.0, -1.0, 1.0), points::new(10.0, 3.0, 6.0), true),
        (points::new(4.0, -3.0, -1.0), points::new(10.0, 3.0, 6.0), false),
        (points::new(6.0, -1.0, 1.0), points::new(12.0, 5.0, 8.0), false),
    ];
    for (min, max, result) in data {
        let bbox2 = Bounds::new(min, max);
        assert_eq!(bbox.contains_box(bbox2), result);
    }
}

#[test]
fn test_bounding_box_transformation() {
    let bbox = Bounds::new(points::new(-1.0, -1.0, -1.0), points::new(1.0, 1.0, 1.0));
    let matrix = rotation_x(math::PI / 4.0) * rotation_y(math::PI / 4.0);
    let bbox = bbox.transform(&matrix);
    assert_eq!(bbox.min.round_items(), points::new(-1.41421, -1.70711, -1.70711));
    assert_eq!(bbox.max.round_items(), points::new(1.41421, 1.70711, 1.70711));
}

/// Get a shape's bounding box in its parent's space
#[test]
fn test_box_in_parent_space() {
    let shape = Shape::sphere().scale(0.5, 2.0, 4.0).translate(1.0, -3.0, 5.0);
    let bbox = shape.parent_space_bounds();
    assert_eq!(bbox.min, points::new(0.5, -5.0, 1.0));
    assert_eq!(bbox.max, points::new(1.5, -1.0, 9.0));
}