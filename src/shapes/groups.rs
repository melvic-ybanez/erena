use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::rays::{Intersection, Intersection3D, Ray};
use crate::shapes::{Geo, Shape, cubes};
use crate::tuples::vectors::Vector;
use crate::tuples::points::Point;
use crate::shapes::bounds::Bounds;
use crate::tuples::points;
use crate::math::Real;
use crate::math;

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    pub children: RefCell<Vec<Rc<Shape>>>,
    bounds: RefCell<Option<Bounds>>
}

impl Group {
    pub fn new(objects: Vec<Rc<Shape>>) -> Group {
        Group { children: RefCell::new(objects), bounds: RefCell::new(None) }
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
        children.into_iter().for_each(|child| self.add_child(Weak::clone(&parent), child));
    }

    pub(crate) fn bounds(&self) -> Bounds {
        let mut cached = self.bounds.borrow_mut();
        if let Some(bounds) = *cached {
            bounds
        } else {
            if self.children.borrow().is_empty() {
                return Bounds::new(Point::origin(), Point::origin());
            }

            let children = self.children.borrow();
            let corners = children.iter()
                .flat_map(|child| {
                    let Bounds { min, max } = child.bounds();

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
                            child.transformation.clone() * points::new(*x, *y, *z)
                        })
                        .collect::<Vec<_>>()
                });

            let get = |point: Option<Point>| point.unwrap_or_else(Point::origin);

            let ord = |a: &Point, b: &Point, f: fn(Point) -> Real| {
                math::order_reals(f(*a), f(*b))
            };
            let ord_x = |a: &Point, b: &Point| ord(a, b, |p| p.x);
            let ord_y = |a: &Point, b: &Point| ord(a, b, |p| p.y);
            let ord_z = |a: &Point, b: &Point| ord(a, b, |p| p.z);

            let min_x = get(corners.clone().min_by(ord_x)).x;
            let min_y = get(corners.clone().min_by(ord_y)).y;
            let min_z = get(corners.clone().min_by(ord_z)).z;
            let max_x = get(corners.clone().max_by(ord_x)).x;
            let max_y = get(corners.clone().max_by(ord_y)).y;
            let max_z = get(corners.clone().max_by(ord_z)).z;

            let bounds = Bounds::new(
                points::new(min_x, min_y, min_z),
                points::new(max_x, max_y, max_z)
            );
            cached.replace(bounds);
            bounds
        }
    }
}

pub fn intersect(shape: &Shape, ray: &Ray) -> Vec<Intersection3D> {
    // If the ray does not intersect with the bounding box,
    // do not bother checking the children
    if cubes::intersect(shape, ray).is_empty() {
        return vec![];
    }

    if let Geo::Group(Group { ref children, .. }) = shape.geo {
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