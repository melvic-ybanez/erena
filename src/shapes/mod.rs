use crate::rays::{Ray, Intersection};
use crate::tuples::points::Point;
use crate::matrix::Matrix;
use crate::tuples::vectors::Vector;

pub trait Shape: Sized {
    fn intersect(&self, ray: Ray) -> Vec<Intersection<Self>>;

    fn transform(&mut self, transformation: Matrix);

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
        let transformation = self.transformation.inverse().expect("Can not inverse transformation");
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

    fn transform(&mut self, transformation: Matrix) {
        self.transformation = Box::new(transformation)
    }

    fn normal_at(&self, point: Point) -> Vector {
        (point - Point::origin()).to_vector().normalize()
    }
}

#[cfg(test)]
mod tests;