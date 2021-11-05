use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
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
        return self.edge1
    }

    pub fn get_edge2(&self) -> Vector {
        return self.edge2
    }

    pub fn get_normal(&self) -> Vector {
        self.normal
    }
}

pub(crate) fn not_a_triangle() {
    panic!("Not a triangle");
}