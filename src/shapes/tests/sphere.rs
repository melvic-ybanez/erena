use crate::rays::Ray;
use crate::tuples::{Point, Vector};
use crate::shapes::{Sphere, Shape};

#[test]
fn test_two_point_intersection() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let xs = Sphere::new().intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);
}

#[test]
fn test_tangent_intersection() {
    let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let xs = Sphere::new().intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
fn test_ray_missing() {
    let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let xs = Sphere::new().intersect(ray);
    assert_eq!(xs.len(), 0);
}

/// Tests a ray originating inside the sphere
#[test]
fn test_ray_originating_inside() {
    let ray = Ray::new(Point::origin(), Vector::new(0.0, 0.0, 1.0));
    let xs = Sphere::new().intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);
}

#[test]
fn test_a_sphere_behind_ray() {
    let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    let xs = Sphere::new().intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -6.0);
    assert_eq!(xs[1].t, -4.0);
}

#[test]
fn test_object_of_intersection() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new();
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object, sphere);
    assert_eq!(xs[1].object, sphere);
}