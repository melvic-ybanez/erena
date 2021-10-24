use crate::shapes::Shape;
use crate::matrix::{Matrix, CanTransform, translation};
use crate::materials::Material;
use crate::rays::Ray;
use crate::tuples::{points, vectors};
use crate::shapes;
use crate::shapes::Space3D::TestShape;

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

    let shape = Shape::test().with_material(mat);
    assert_eq!(shape.material, mat);
}

/// Tests intersecting a scaled shape with a ray
#[test]
fn test_intersection_with_scale() {
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let shape = Shape::test().scale(2.0, 2.0, 2.0);
    shape.intersect(&ray);

    assert_eq!(TestShape, shape.shape);
    unsafe {
        if let Some(ray) = shapes::test::SAVED_RAY {
            assert_eq!(ray.origin, points::new(0.0, 0.0, -2.5));
            assert_eq!(ray.direction, vectors::new(0.0, 0.0, 0.5));
        } else {
            assert!(false);
        }
    }
}

/// Tests intersecting a translated shape with a ray
#[test]
fn test_intersection_with_translation() {
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let shape = Shape::test().translate(5.0, 0.0, 0.0);
    shape.intersect(&ray);

    assert_eq!(TestShape, shape.shape);
    unsafe {
        if let Some(ray) = shapes::test::SAVED_RAY {
            assert_eq!(ray.origin, points::new(-5.0, 0.0, -5.0));
            assert_eq!(ray.direction, vectors::new(0.0, 0.0, 1.0));
        } else {
            assert!(false);
        }
    }
}