use crate::rays::Ray;
use crate::tuples::{Point, Vector};
use crate::shapes::{Sphere, Shape};

#[test]
fn test_two_point_intersection() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new();
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 4.0);
    assert_eq!(xs[1], 6.0);
}

#[test]
fn test_tangent_intersection() {
    let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new();
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 5.0);
    assert_eq!(xs[1], 5.0);
}

#[test]
fn test_ray_missing() {
    let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new();
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 0);
}

/// Tests a ray originating inside the sphere
#[test]
fn test_ray_originating_inside() {
    let ray = Ray::new(Point::origin(), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new();
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], -1.0);
    assert_eq!(xs[1], 1.0);
}

#[test]
fn test_a_sphere_behind_ray() {
    let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new();
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], -6.0);
    assert_eq!(xs[1], -4.0);
}