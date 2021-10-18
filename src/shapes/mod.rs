use crate::rays::{Ray, Intersection};
use crate::tuples::points::Point;
use crate::matrix::Matrix;
use crate::tuples::vectors::Vector;

pub trait Shape: Sized {
    fn intersect(&self, ray: Ray) -> Vec<Intersection<Self>>;

    fn transform(&mut self, transformation: Matrix) -> &Self;

    fn normal_at(&self, point: Point) -> Vector;
}

#[derive(Debug, PartialEq,  Clone)]
pub struct Sphere {
    pub transformation: Box<Matrix>,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere { transformation: Box::new(Matrix::id44()) }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: Ray) -> Vec<Intersection<'_, Sphere>> {
        let transformation = self.transformation.inverse_or_id44();
        let ray = ray.transform(&transformation);

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

    fn transform(&mut self, transformation: Matrix) -> &Self {
        self.transformation = Box::new(transformation);
        self
    }

    fn normal_at(&self, world_point: Point) -> Vector {
        let inverse = self.transformation.inverse_or_id44();
        let object_point = &inverse * world_point;
        let object_normal = object_point - Point::origin();
        let world_normal = inverse.transpose() * object_normal;
        world_normal.to_vector().normalize()
    }
}

#[cfg(test)]
mod tests;