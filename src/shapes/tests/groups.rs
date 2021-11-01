use crate::shapes::{Shape, Geo, groups};
use crate::matrix::{Matrix, CanTransform};
use crate::shapes::groups::not_a_group;
use crate::shapes::arena::Arena;
use crate::rays::Ray;
use crate::tuples::points::Point;
use crate::tuples::{vectors, points};

#[test]
fn test_create_group() {
    let group = Shape::empty_group();
    assert_eq!(group.transformation, Matrix::id44());
    if let Geo::Group(group) = group.geo {
        assert!(group.is_empty());
    } else {
        not_a_group();
    }
}

#[test]
fn test_shape_parent() {
    let shape = Shape::test();
    assert!(shape.parent.is_none());
}

#[test]
fn test_add_child() {
    let mut arena = Arena::new();
    let mut group = Shape::empty_group();
    let mut shape = Shape::test();

    arena.connect(&mut group, &mut shape);

    if let Geo::Group(group) = group.geo {
        assert!(group.non_empty());
        assert!(group.contains(&shape));
    } else {
        not_a_group();
    }
}

/// Tests intersection of a ray with an empty group
#[test]
fn test_intersect_empty() {
    let group = Shape::empty_group();
    let ray = Ray::new(Point::origin(), vectors::new(0.0, 0.0, 1.0));
    let xs = groups::intersect(&group, &ray, &mut Arena::new());
    assert!(xs.is_empty());
}

/// Tests intersection of a ray with a non-empty group.
/// The ray intersects two of the group's children.
#[test]
fn test_intersect_non_empty() {
    let mut arena = Arena::new();
    let mut group = Shape::empty_group();
    let mut s1 = Shape::sphere();
    let mut s2 = Shape::sphere().translate(0.0, 0.0, -3.0);
    let mut s3 = Shape::sphere().translate(5.0, 0.0, 0.0);

    arena.connect(&mut group, &mut s1);
    arena.connect(&mut group, &mut s2);
    arena.connect(&mut group, &mut s3);

    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let xs = groups::intersect(&group, &ray, &mut arena);

    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].object, &s2);
    assert_eq!(xs[1].object, &s2);
    assert_eq!(xs[2].object, &s1);
    assert_eq!(xs[3].object, &s1);
}