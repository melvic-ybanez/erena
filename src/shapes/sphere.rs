use crate::rays::{Intersection, Ray};
use crate::shapes::{Shape, Space3D};
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;

pub fn intersect<'a>(sphere: &'a Shape, ray: &Ray) -> Vec<Intersection<'a, Space3D>> {
    // note: sphere's center is at world origin
    let sphere_to_ray = ray.origin - Point::origin();

    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * ray.direction.dot(sphere_to_ray);
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

pub fn normal_at(sphere: &Shape, world_point: Point) -> Vector {
    let inverse = sphere.transformation.inverse_or_id44();
    let object_point = &inverse * world_point;
    let object_normal = object_point - Point::origin();
    let world_normal = inverse.transpose() * object_normal;
    world_normal.to_vector().normalize()
}