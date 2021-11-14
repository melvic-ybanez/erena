use crate::tuples::points::Point;
use crate::shapes::{Shape, Geo};
use crate::tuples::points;
use crate::math::Real;
use crate::shapes::cylinders::CylLike;
use std::ops::Add;

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
        between(point.x, self.min.x, self.max.x) &&
            between(point.y, self.min.y, self.max.y) &&
            between(point.z, self.min.z, self.max.z)
    }

    pub fn contains_box(&self, other: Bounds) -> bool {
        self.contains_point(other.min) && self.contains_point(other.max)
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