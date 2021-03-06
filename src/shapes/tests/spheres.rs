use std::f64::consts::{FRAC_1_SQRT_2, PI};

use crate::materials::Material;
use crate::matrix::{rotation_z, scaling, translation, CanTransform, Matrix};
use crate::rays::Ray;
use crate::shapes::{spheres, Shape};
use crate::tuples::points::Point;
use crate::tuples::{points, vectors};

#[test]
fn test_two_point_intersection() {
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let sphere = Shape::sphere();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);
}

#[test]
fn test_tangent_intersection() {
    let ray = Ray::new(points::new(0.0, 1.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let sphere = Shape::sphere();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
fn test_ray_missing() {
    let ray = Ray::new(points::new(0.0, 2.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let sphere = Shape::sphere();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 0);
}

/// Tests a ray originating inside the sphere
#[test]
fn test_ray_originating_inside() {
    let ray = Ray::new(Point::origin(), vectors::new(0.0, 0.0, 1.0));
    let sphere = Shape::sphere();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);
}

#[test]
fn test_a_sphere_behind_ray() {
    let ray = Ray::new(points::new(0.0, 0.0, 5.0), vectors::new(0.0, 0.0, 1.0));
    let sphere = Shape::sphere();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -6.0);
    assert_eq!(xs[1].t, -4.0);
}

#[test]
fn test_object_of_intersection() {
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let sphere = Shape::sphere();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(*xs[0].object, sphere);
    assert_eq!(*xs[1].object, sphere);
}

#[test]
fn test_default_transformation() {
    let sphere = Shape::sphere();
    assert_eq!(sphere.transformation, Matrix::id44());
}

#[test]
fn test_intersect_with_scaled_sphere() {
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let sphere = Shape::sphere().transform(scaling(2.0, 2.0, 2.0));
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);
}

#[test]
fn test_intersect_with_translated_sphere() {
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let sphere = Shape::sphere().transform(translation(5.0, 0.0, 0.0));
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 0);
}

#[test]
fn test_normal_at_point_on_x_axis() {
    let sphere = Shape::sphere();
    let n = sphere.default_normal_at(points::new(1.0, 0.0, 0.0));
    assert_eq!(n, vectors::new(1.0, 0.0, 0.0));
}

#[test]
fn test_normal_at_point_on_y_axis() {
    let sphere = Shape::sphere();
    let n = sphere.default_normal_at(points::new(0.0, 1.0, 0.0));
    assert_eq!(n, vectors::new(0.0, 1.0, 0.0));
}

#[test]
fn test_normal_at_point_on_z_axis() {
    let sphere = Shape::sphere();
    let n = sphere.default_normal_at(points::new(0.0, 0.0, 1.0));
    assert_eq!(n, vectors::new(0.0, 0.0, 1.0));
}

#[test]
fn test_normal_at_nonaxial_point() {
    let sphere = Shape::sphere();
    let component = 3_f64.sqrt() / 3.0;
    let n = sphere.default_normal_at(points::new(component, component, component));
    assert_eq!(n, vectors::new(component, component, component));
}

#[test]
fn test_normal_is_normalized() {
    let sphere = Shape::sphere();
    let component = 3_f64.sqrt() / 3.0;
    let n = sphere.default_normal_at(points::new(component, component, component));
    assert_eq!(n, n.normalize());
}

#[test]
fn test_normal_on_translated_sphere() {
    let sphere = Shape::sphere().transform(translation(0.0, 1.0, 0.0));
    let n = sphere.default_normal_at(points::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    assert_eq!(n, vectors::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
}

#[test]
fn test_normal_on_transformed_sphere() {
    let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
    let sphere = Shape::sphere().transform(m);
    let n = sphere.default_normal_at(points::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));
    assert_eq!(n.round_items(), vectors::new(0.0, 0.97014, -0.24254));
}

#[test]
fn test_default_material() {
    let sphere = Shape::sphere();
    let mat = Material::default();
    assert_eq!(sphere.material, mat);
}

#[test]
fn test_material_update() {
    let mut mat = Material::default();
    mat.ambient = 1.0;
    let sphere = Shape::sphere().material_ref(&mat);
    assert_eq!(sphere.material, mat);
}

#[test]
fn test_glass_sphere() {
    let s = spheres::glass();
    assert_eq!(s.transformation, Matrix::id44());
    assert_eq!(s.material.transparency, 1.0);
    assert_eq!(s.material.refractive_index, 1.5);
}
