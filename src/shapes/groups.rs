use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::rays::{Intersection, Intersection3D, Ray};
use crate::shapes::{Geo, Shape};
use crate::tuples::vectors::Vector;
use crate::tuples::points::Point;

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    pub children: RefCell<Vec<Rc<Shape>>>
}

impl Group {
    pub fn new(objects: Vec<Rc<Shape>>) -> Group {
        Group { children: RefCell::new(objects) }
    }

    pub fn empty() -> Group {
        Group::new(vec![])
    }

    pub fn contains(&self, shape: Rc<Shape>) -> bool {
        self.children.borrow().contains(&shape)
    }

    pub fn is_empty(&self) -> bool {
        self.children.borrow().is_empty()
    }

    pub fn non_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn add_child(&self, parent: Weak<Shape>, child: Rc<Shape>) {
        self.children.borrow_mut().push(Rc::clone(&child));
        child.set_parent(parent);
    }
}

pub fn intersect(shape: &Shape, ray: &Ray) -> Vec<Intersection3D> {
    if let Geo::Group(Group { ref children }) = shape.geo {
        let mut xs: Vec<_> = children.borrow()
            .iter()
            .flat_map(|child| child.intersect(ray))
            .collect();
        xs.sort_by(Intersection::compare);
        xs
    } else {
        vec![]
    }
}

/// Probably never gonna be needed.
pub fn normal_at() -> Vector {
    Vector::zero()
}

pub fn not_a_group() {
    panic!("Not a group");
}