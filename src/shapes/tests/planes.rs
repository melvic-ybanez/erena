use crate::rays::Ray;
use crate::shapes::{Geo, Shape};
use crate::tuples::points::Point;
use crate::tuples::{points, vectors};
use std::rc::Rc;

/// The normal of a plane is the same everywhere
#[test]
fn test_normal_at() {
    let plane = Shape::plane();
    let n1 = plane.default_normal_at(Point::origin());
    let n2 = plane.default_normal_at(points::new(10.0, 0.0, -10.0));
    let n3 = plane.default_normal_at(points::new(-5.0, 0.0, 150.0));

    assert_eq!(plane.geo, Geo::Plane);
    assert_eq!(n1, vectors::new(0.0, 1.0, 0.0));
    assert_eq!(n2, vectors::new(0.0, 1.0, 0.0));
    assert_eq!(n3, vectors::new(0.0, 1.0, 0.0));
}

/// Intersect with a ray parallel to the plane
#[test]
fn test_intersect_parallel() {
    let plane = Shape::plane();
    let ray = Ray::new(points::new(0.0, 10.0, 0.0), vectors::new(0.0, 0.0, 1.0));
    let xs = plane.intersect(&ray);
    assert!(xs.is_empty());
}

/// Intersect with a coplanar ray
#[test]
fn test_intersect_coplanar() {
    let plane = Shape::plane();
    let ray = Ray::new(Point::origin(), vectors::new(0.0, 0.0, 1.0));
    let xs = plane.intersect(&ray);
    assert!(xs.is_empty());
}

/// A ray intersecting a plane from above
#[test]
fn test_intersect_from_above() {
    let plane = Rc::new(Shape::plane());
    let ray = Ray::new(points::new(0.0, 1.0, 0.0), vectors::new(0.0, -1.0, 0.0));
    let xs = plane.intersect(&ray);

    assert_eq!(xs.len(), 1);
    assert_eq!(xs[0].t, 1.0);
    assert_eq!(xs[0].object, plane);
}

#[test]
fn test_intersect_from_below() {
    let plane = Rc::new(Shape::plane());
    let ray = Ray::new(points::new(0.0, -1.0, 0.0), vectors::new(0.0, 1.0, 0.0));
    let xs = plane.intersect(&ray);

    assert_eq!(xs.len(), 1);
    assert_eq!(xs[0].t, 1.0);
    assert_eq!(xs[0].object, plane);
}
