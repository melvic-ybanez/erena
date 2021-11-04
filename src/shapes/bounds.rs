use crate::tuples::points::Point;
use crate::shapes::{Shape, Geo};
use crate::tuples::points;
use crate::math::Real;
use crate::shapes::cylinders::CylLike;

pub struct Bounds {
    pub min: Point,
    pub max: Point
}

impl Bounds {
    pub fn new(min: Point, max: Point) -> Bounds {
        Bounds { min, max }
    }

    /// Constructs a bounds with opposing minimum and maximum
    pub fn from_min(min: Point) -> Bounds {
        Bounds::new(min, (-min).to_point())
    }

    pub fn of(shape: &Shape) -> Bounds {
        match &shape.geo {
            Geo::Sphere => Bounds::from_min(points::new(-1.0, -1.0, -1.0)),
            Geo::TestShape => Bounds::from_min(Point::origin()),
            Geo::Plane => Bounds::from_min(points::new(-Real::INFINITY, -1.0, -Real::INFINITY)),
            Geo::Cube => Bounds::of(&Shape::cube()),
            Geo::Cylinder(_) => {
                let min = if let Geo::Cylinder(CylLike { min, .. }) = shape.geo {
                    min
                } else {
                    -Real::INFINITY
                };
                Bounds::from_min(points::new(-1.0, min, -1.0))
            },
            Geo::Group(group) => group.bounds(shape)
        }
    }
}