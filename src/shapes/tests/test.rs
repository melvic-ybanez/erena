use crate::shapes::{Shape, Geo};
use crate::matrix::{Matrix, CanTransform, translation};
use crate::materials::Material;
use crate::rays::Ray;
use crate::tuples::{points, vectors};
use crate::{shapes, math};
use std::f64::consts::FRAC_1_SQRT_2;

#[test]
fn test_default_transformation() {
    let shape = Shape::test();
    assert_eq!(shape.transformation, Matrix::id44());
}

#[test]
fn test_set_transformation() {
    let shape = Shape::test().translate(2.0, 3.0, 4.0);
    assert_eq!(shape.transformation, translation(2.0, 3.0, 4.0));
}

#[test]
fn test_default_material() {
    let shape = Shape::test();
    assert_eq!(Material::default(), shape.material);
}

#[test]
fn test_set_material() {
    let mut mat = Material::default();
    mat.ambient = 1.0;

    let shape = Shape::test().material_ref(&mat);
    assert_eq!(shape.material, mat);
}

/// Tests intersecting a scaled shape with a ray
#[test]
fn test_intersection_with_scale() {
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let shape = Shape::test().scale(2.0, 2.0, 2.0);
    shape.intersect(&ray);

    assert_eq!(Geo::TestShape, shape.geo);
    unsafe {
        if let Some(ray) = shapes::test::SAVED_RAY {
            assert_eq!(ray.origin, points::new(0.0, 0.0, -2.5));
            assert_eq!(ray.direction, vectors::new(0.0, 0.0, 0.5));
        } else {
            panic!("No saved ray");
        }
    }
}

/// Tests intersecting a translated shape with a ray
#[test]
fn test_intersection_with_translation() {
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let shape = Shape::test().translate(5.0, 0.0, 0.0);
    shape.intersect(&ray);

    assert_eq!(Geo::TestShape, shape.geo);
    unsafe {
        if let Some(ray) = shapes::test::SAVED_RAY {
            assert_eq!(ray.origin, points::new(-5.0, 0.0, -5.0));
            assert_eq!(ray.direction, vectors::new(0.0, 0.0, 1.0));
        } else {
            panic!("No saved ray");
        }
    }
}

#[test]
fn test_normal_on_translated_shape() {
    let shape = Shape::test().translate(0.0, 1.0, 0.0);
    let n = shape.default_normal_at(points::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    assert_eq!(n, vectors::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
}

#[test]
fn test_normal_on_transformed_shape() {
    let shape = Shape::test().rotate_z(math::PI / 5.0).scale(1.0, 0.5, 1.0);
    let n = shape.default_normal_at(points::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));
    assert_eq!(n.round_items(), vectors::new(0.0, 0.97014, -0.24254));
}