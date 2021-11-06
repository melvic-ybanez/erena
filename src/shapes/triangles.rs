use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::shapes::Shape;
use crate::rays::{Intersection3D, Ray, Intersection};
use crate::math;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    edge1: Vector,
    edge2: Vector,
    normal: Vector,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
        let edge1 = (p2 - p1).to_vector();
        let edge2 = (p3 - p1).to_vector();
        Triangle { p1, p2, p3, edge1, edge2, normal: edge2.cross(edge1).normalize(), }
    }

    pub fn get_edge1(&self) -> Vector {
        self.edge1
    }

    pub fn get_edge2(&self) -> Vector {
        self.edge2
    }

    pub fn get_normal(&self) -> Vector {
        self.normal
    }

    pub fn get_p1(&self) -> Point {
        self.p1
    }

    pub fn get_p2(&self) -> Point {
        self.p2
    }

    pub fn get_p3(&self) -> Point {
        self.p3
    }

    /// Computes the ray-triangle intersection based on the Moller-Trumbore
    /// algorithm
    pub fn intersect(&self, shape: &Shape, ray: &Ray) -> Vec<Intersection3D> {
        let dir_cross_e2 = ray.direction.cross(self.get_edge2());
        let determinant = self.get_edge1().dot(dir_cross_e2);

        if determinant.abs() < math::EPSILON {
            return vec![]
        }

        let f = 1.0 / determinant;
        let p1_to_origin = ray.origin - self.p1;
        let u = f * p1_to_origin.dot(dir_cross_e2);

        if u < 0.0 || u > 1.0 {
            return vec![]
        }

        vec![Intersection::new(1.0, Rc::new(shape.clone()))]
    }
}