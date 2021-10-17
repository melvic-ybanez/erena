use crate::rays::Ray;
use crate::shapes::{Sphere, Shape};
use crate::tuples::{points, vectors};
use crate::tuples::points::Point;
use crate::matrix::{Matrix, scaling, translation};

#[test]
fn test_two_point_intersection() {
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new();
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);
}

#[test]
fn test_tangent_intersection() {
    let ray = Ray::new(points::new(0.0, 1.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new();
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
fn test_ray_missing() {
    let ray = Ray::new(points::new(0.0, 2.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new();
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 0);
}

/// Tests a ray originating inside the sphere
#[test]
fn test_ray_originating_inside() {
    let ray = Ray::new(Point::origin(), vectors::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new();
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);
}

#[test]
fn test_a_sphere_behind_ray() {
    let ray = Ray::new(points::new(0.0, 0.0, 5.0), vectors::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new();
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -6.0);
    assert_eq!(xs[1].t, -4.0);
}

#[test]
fn test_object_of_intersection() {
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new();
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(*xs[0].object, sphere);
    assert_eq!(*xs[1].object, sphere);
}

#[test]
fn test_default_transformation() {
    let sphere = Sphere::new();
    assert_eq!(*sphere.transformation, Matrix::id44());
}

#[test]
fn test_intersect_with_scaled_sphere() {
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let mut sphere = Sphere::new();
    sphere.transform(scaling(2.0, 2.0, 2.0));
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);
}

#[test]
fn test_intersect_with_translated_sphere() {
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let mut sphere = Sphere::new();
    sphere.transform(translation(5.0, 0.0, 0.0));
    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 0);
}

#[test]
fn test_normal_at_point_on_x_axis() {
    let sphere = Sphere::new();
    let n = sphere.normal_at(points::new(1.0, 0.0, 0.0));
    assert_eq!(n, vectors::new(1.0, 0.0, 0.0));
}

#[test]
fn test_normal_at_point_on_y_axis() {
    let sphere = Sphere::new();
    let n = sphere.normal_at(points::new(0.0, 1.0, 0.0));
    assert_eq!(n, vectors::new(0.0, 1.0, 0.0));
}

#[test]
fn test_normal_at_point_on_z_axis() {
    let sphere = Sphere::new();
    let n = sphere.normal_at(points::new(0.0, 0.0, 1.0));
    assert_eq!(n, vectors::new(0.0, 0.0, 1.0));
}

#[test]
fn test_normal_at_nonaxial_point() {
    let sphere = Sphere::new();
    let component = 3_f64.sqrt() / 3.0;
    let n = sphere.normal_at(points::new(component, component, component));
    assert_eq!(n, vectors::new(component, component, component));
}

#[test]
fn test_normal_is_normalized() {
    let sphere = Sphere::new();
    let component = 3_f64.sqrt() / 3.0;
    let n = sphere.normal_at(points::new(component, component, component));
    assert_eq!(n, n.normalize());
}