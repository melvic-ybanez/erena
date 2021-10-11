use crate::rays::{Ray, Intersection};
use crate::tuples::Point;

pub trait Shape: Sized {
    fn intersect(&self, ray: Ray) -> Vec<Intersection<Self>>;
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Sphere {}

impl Sphere {
    pub(crate) fn new() -> Sphere {
        Sphere {}
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: Ray) -> Vec<Intersection<Sphere>> {
        // note: sphere's center is at world origin
        let sphere_to_ray = ray.origin.0 - Point::origin().0;

        let a = ray.direction.0.dot(ray.direction.0);
        let b = 2.0 * ray.direction.0.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            vec![
                Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), *self),     // t1
                Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), *self),     // t2
            ]
        }
    }
}

#[cfg(test)]
mod tests;