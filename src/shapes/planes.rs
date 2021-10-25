use crate::tuples::vectors::Vector;
use crate::tuples::vectors;
use crate::shapes::Shape;
use crate::rays::{Ray, Intersection3D};
use crate::math;

/// Computes the normal-at function for a plane.
/// Note: The plane has no curvature, so it's normal vector
/// should be the same regardless of the location
pub fn normal_at() -> Vector {
    vectors::new(0.0, 1.0, 0.0)
}

pub fn intersect<'a>(plane: &'a Shape, ray: &Ray) -> Vec<Intersection3D<'a>> {
    if ray.direction.y.abs() < math::EPSILON {
        vec![]
    } else {
        let t = -ray.origin.y / ray.direction.y;
        vec![Intersection3D::new(t, plane)]
    }
}