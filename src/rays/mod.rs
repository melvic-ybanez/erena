mod intersections;
mod comps;

use crate::math::Real;
pub use intersections::Intersection;
pub use comps::*;
use crate::matrix::Matrix;
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Ray {
    pub(crate) origin: Point,
    pub(crate) direction: Vector
}

impl Ray {
    pub(crate) fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub(crate) fn position(&self, t: Real) -> Point {
        self.origin + self.direction * t
    }

    pub(crate) fn transform(&self, transformation: &Matrix) -> Ray {
        Ray::new(transformation.clone() * self.origin, transformation.clone() * self.direction)
    }
}

#[cfg(test)]
mod tests {
    use crate::rays::Ray;
    use crate::matrix::{translation, scaling};
    use crate::tuples::{points, vectors};

    #[test]
    fn test_creating_ray() {
        let origin = points::new(1.0, 2.0, 3.0);
        let direction = vectors::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    /// Tests computing a point from a distance
    #[test]
    fn test_compute_point_from_distance() {
        let ray = Ray::new(points::new(2.0, 3.0, 4.0), vectors::new(1.0, 0.0, 0.0));
        assert_eq!(ray.position(0.0), points::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), points::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), points::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), points::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn test_ray_translation() {
        let ray = Ray::new(points::new(1.0, 2.0, 3.0), vectors::new(0.0, 1.0, 0.0));
        let matrix = translation(3.0, 4.0, 5.0);
        let ray = ray.transform(&matrix);
        assert_eq!(ray.origin, points::new(4.0, 6.0, 8.0));
        assert_eq!(ray.direction, vectors::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_ray_scaling() {
        let ray = Ray::new(points::new(1.0, 2.0, 3.0), vectors::new(0.0, 1.0, 0.0));
        let matrix = scaling(2.0, 3.0, 4.0);
        let ray = ray.transform(&matrix);
        assert_eq!(ray.origin, points::new(2.0, 6.0, 12.0));
        assert_eq!(ray.direction, vectors::new(0.0, 3.0, 0.0));
    }
}