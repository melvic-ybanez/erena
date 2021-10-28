use crate::rays::{Intersection, Ray, Intersection3D};
use crate::shapes::{Shape, Space3D};
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::materials::Material;

pub fn intersect<'a>(sphere: &'a Shape, transformed_ray: &Ray) -> Vec<Intersection3D<'a>> {
    // note: sphere's center is at world origin
    let sphere_to_ray = transformed_ray.origin - Point::origin();

    let a = transformed_ray.direction.dot(transformed_ray.direction);
    let b = 2.0 * transformed_ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
    let discriminant = b.powf(2.0) - 4.0 * a * c;

    if discriminant < 0.0 {
        vec![]
    } else {
        vec![
            Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), sphere),     // t1
            Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), sphere),     // t2
        ]
    }
}

pub fn normal_at(local_point: Point) -> Vector {
    (local_point - Point::origin()).to_vector()
}

pub fn glass() -> Shape {
    Shape::sphere().material(
        Material::default().transparency(1.0).refractive_index(1.5)
    )
}