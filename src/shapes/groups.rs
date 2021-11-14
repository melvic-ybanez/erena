use std::cell::RefCell;
use std::rc::{Rc, Weak};



use crate::rays::{Intersection, Intersection3D, Ray};
use crate::shapes::bounds::Bounds;
use crate::shapes::{cubes, Shape};


use crate::tuples::vectors::Vector;

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    pub children: RefCell<Vec<Rc<Shape>>>,
    bounds: RefCell<Option<Bounds>>,
}

impl Group {
    pub fn new(objects: Vec<Rc<Shape>>) -> Group {
        Group {
            children: RefCell::new(objects),
            bounds: RefCell::new(None),
        }
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

    pub fn add_children(&self, parent: Weak<Shape>, children: Vec<Rc<Shape>>) {
        children
            .into_iter()
            .for_each(|child| self.add_child(Weak::clone(&parent), child));
    }

    pub(crate) fn bounds(&self) -> Bounds {
        let mut cached = self.bounds.borrow_mut();
        if let Some(bounds) = *cached {
            bounds
        } else {
            let bounds = self
                .children
                .borrow()
                .iter()
                .fold(Bounds::empty(), |bounds, child| {
                    bounds + child.parent_space_bounds()
                });
            cached.replace(bounds);
            bounds
        }
    }

    pub fn intersect(&self, shape: &Shape, ray: &Ray) -> Vec<Intersection3D> {
        // If the ray does not intersect with the bounding box,
        // do not bother checking the children
        if cubes::intersect(shape, ray).is_empty() {
            return vec![];
        }

        let mut xs: Vec<_> = self
            .children
            .borrow()
            .iter()
            .flat_map(|child| child.intersect(ray))
            .collect();
        xs.sort_by(Intersection::compare);
        xs
    }

    pub fn get_child(&self, i: usize) -> Shape {
        (*self.children.borrow()[i]).clone()
    }
}

/// Probably never gonna be needed.
pub fn normal_at() -> Vector {
    panic!("Groups have no normal vector")
}

pub fn not_a_group() {
    panic!("Not a group");
}
