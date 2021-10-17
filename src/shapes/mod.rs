use crate::rays::{Ray, Intersection};
use crate::tuples::points::Point;
use crate::matrix::Matrix;

pub trait Shape: Sized {
    fn intersect(&self, ray: Ray) -> Vec<Intersection<Self>>;
}

#[derive(Debug, PartialEq,  Clone)]
pub struct Sphere {
    pub transformation: Box<Matrix>,
}

impl Sphere {
    pub(crate) fn new() -> Sphere {
        Sphere { transformation: Box::new(Matrix::id44()) }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: Ray) -> Vec<Intersection<'_, Sphere>> {
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
                Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), self),     // t1
                Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), self),     // t2
            ]
        }
    }
}

#[cfg(test)]
mod tests;