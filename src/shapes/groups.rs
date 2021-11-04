use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::rays::{Intersection, Intersection3D, Ray};
use crate::shapes::{Geo, Shape};
use crate::tuples::vectors::Vector;
use crate::tuples::points::Point;
use crate::shapes::bounds::Bounds;
use crate::tuples::points;
use crate::math::Real;

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

    pub(crate) fn bounds(&self, object: &Shape) -> Bounds {
        let Bounds { min, max } = object.bounds();

        // transform corners. Note that we are using the left-hand rule
        // so "back" here means negative z-axis (or towards the user)
        let corners = [
            (min.x, min.y, min.z),  // lower left back
            (max.x, min.y, min.z),  // lower right back
            (min.x, min.y, max.z),  // lower left front
            (max.x, min.y, max.z),  // lower right front
            (min.x, max.y, min.z),  // upper left back
            (max.x, max.y, min.z),  // upper right back
            (min.x, max.y, max.z),  // upper left front
            (max.x, max.y, max.z)   // upper right front
        ];

        corners.iter()
            .map(|(x, y, z)| {
                object.transformation.clone() * points::new(*x, *y, *z)
            })
            .fold(Bounds::new(Point::origin(), Point::origin()), |res, corner| {
                let min = points::new(
                    Real::min(res.min.x, corner.x),
                    Real::min(res.min.y, corner.y),
                    Real::min(res.min.z, corner.z)
                );
                let max = points::new(
                    Real::max(res.max.x, corner.x),
                    Real::max(res.max.y, corner.y),
                    Real::max(res.max.z, corner.z)
                );
                Bounds::new(min, max)
            })
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
    panic!("Groups have no normal vector")
}

pub fn not_a_group() {
    panic!("Not a group");
}