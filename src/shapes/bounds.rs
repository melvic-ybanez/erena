use crate::tuples::points::Point;
use crate::shapes::{Shape, Geo};
use crate::tuples::points;
use crate::math::Real;

use std::ops::Add;
use crate::matrix::Matrix;

/// Represents a bounding box
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Bounds {
    pub fn new(min: Point, max: Point) -> Bounds {
        Bounds { min, max }
    }

    /// Creates a box that has no space. Since the min points at positive infinity
    /// and the max points at negative infinity, this box is considered invalid.
    pub fn empty() -> Bounds {
        Bounds::from_min(points::new(Real::INFINITY, Real::INFINITY, Real::INFINITY))
    }

    /// Constructs a bounds with opposing minimum and maximum
    pub fn from_min(min: Point) -> Bounds {
        Bounds::new(min, (-min).to_point())
    }

    pub fn of(shape: &Shape) -> Bounds {
        match &shape.geo {
            Geo::Sphere => Bounds::of(&Shape::cube()),
            Geo::Plane => Bounds::from_min(points::new(-Real::INFINITY, 0.0, -Real::INFINITY)),
            Geo::Cube => Bounds::from_min(points::new(-1.0, -1.0, -1.0)),
            Geo::Cylinder(cyl) => cyl.bounds(),
            Geo::Group(group) => group.bounds(),
            Geo::Triangle(tri) => tri.bounds(),
            Geo::TestShape => Bounds::from_min(points::new(-1.0, -1.0, -1.0)),
        }
    }

    pub fn contains_point(&self, point: Point) -> bool {
        let between = |p, min, max| p >= min && p <= max;
        let Bounds { min, max } = self;
        between(point.x, min.x, max.x) &&
            between(point.y, min.y, max.y) &&
            between(point.z, min.z, max.z)
    }

    pub fn contains_box(&self, other: Bounds) -> bool {
        self.contains_point(other.min) && self.contains_point(other.max)
    }

    pub fn transform(&self, matrix: &Matrix) -> Bounds {
        let Bounds { min, max } = self;

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

        corners.iter().fold(Bounds::empty(), |bounds, (x, y, z)| {
            bounds + (matrix.clone() * points::new(*x, *y, *z))
        })
    }
}

impl Add<Point> for Bounds {
    type Output = Bounds;

    fn add(self, rhs: Point) -> Self::Output {
        Bounds::new(
            points::new(
                Real::min(self.min.x, rhs.x),
                Real::min(self.min.y, rhs.y),
                Real::min(self.min.z, rhs.z),
            ),
            points::new(
                Real::max(self.max.x, rhs.x),
                Real::max(self.max.y, rhs.y),
                Real::max(self.max.z, rhs.z),
            ),
        )
    }
}

impl Add for Bounds {
    type Output = Bounds;

    fn add(self, other: Self) -> Self::Output {
        self + other.min + other.max
    }
}