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
    pub kind: TriangleKind,
}

impl Triangle {
    pub fn new<F>(p1: Point, p2: Point, p3: Point, kind_f: F) -> Triangle
    where F: FnOnce(Vector, Vector) -> TriangleKind {
        let edge1 = (p2 - p1).to_vector();
        let edge2 = (p3 - p1).to_vector();
        Triangle {
            p1, p2, p3, edge1, edge2, kind: kind_f(edge1, edge2),
        }
    }

    pub fn regular(p1: Point, p2: Point, p3: Point) -> Triangle {
        Triangle::new(p1, p2, p3, |edge1, edge2| TriangleKind::regular(edge2.cross(edge1).normalize()))
    }

    pub fn smooth(p1: Point, p2: Point, p3: Point, n1: Vector, n2: Vector, n3: Vector) -> Triangle {
        Triangle::new(p1, p2, p3, |_, _| TriangleKind::smooth(n1, n2, n3))
    }

    pub fn get_edge1(&self) -> Vector {
        self.edge1
    }

    pub fn get_edge2(&self) -> Vector {
        self.edge2
    }

    pub fn get_normal(&self) -> Vector {
        match self.kind {
            TriangleKind::Regular { normal } => normal,
            TriangleKind::Smooth(Smooth { n1, .. }) => n1,
        }
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
            return vec![]   // the ray misses
        }

        let origin_cross_e1 = p1_to_origin.to_vector().cross(self.edge1);
        let v = f * ray.direction.dot(origin_cross_e1);

        if v < 0.0 || (u + v) > 1.0 {
            return vec![]
        }

        let t = f * self.edge2.dot(origin_cross_e1);
        vec![Intersection::new(t, Rc::new(shape.clone()))]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TriangleKind {
    Regular { normal: Vector },
    Smooth(Smooth)
}

impl TriangleKind {
    pub fn regular(normal: Vector) -> TriangleKind {
        TriangleKind::Regular { normal }
    }

    pub fn smooth(n1: Vector, n2: Vector, n3: Vector) -> TriangleKind {
        TriangleKind::Smooth(Smooth::new(n1, n2, n3))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Smooth {
    pub n1: Vector,
    pub n2: Vector,
    pub n3: Vector
}

impl Smooth {
    pub fn new(n1: Vector, n2: Vector, n3: Vector) -> Smooth {
        Smooth { n1, n2, n3 }
    }
}