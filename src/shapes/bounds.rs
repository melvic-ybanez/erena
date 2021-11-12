use crate::tuples::points::Point;
use crate::shapes::{Shape, Geo};
use crate::tuples::points;
use crate::math::Real;
use crate::shapes::cylinders::CylLike;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
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
            Geo::Sphere => Bounds::of(&Shape::cube()),
            Geo::TestShape => Bounds::from_min(Point::origin()),
            Geo::Plane => Bounds::from_min(points::new(-Real::INFINITY, -1.0, -Real::INFINITY)),
            Geo::Cube => Bounds::from_min(points::new(-1.0, -1.0, -1.0)),
            Geo::Cylinder(CylLike { min, max, .. }) => Bounds::new(
                points::new(-1.0, *min, -1.0),
                points::new(1.0, *max, 1.0),
            ),
            Geo::Group(group) => group.bounds(),
            Geo::Triangle(tri) => tri.bounds(),
        }
    }
}

// TODO: Add more bounding box tests here.
#[cfg(test)]
mod tests {
    use crate::tuples::points;
    use crate::shapes::Shape;

    #[test]
    fn test_triangle_bounding_box() {
        let p1 = points::new(-3.0, 7.0, 2.0);
        let p2 = points::new(6.0, 2.0, -4.0);
        let p3 = points::new(2.0, -1.0, -1.0);

        let shape = Shape::triangle(p1, p2, p3);
        let bounds = shape.bounds();
        assert_eq!(bounds.min, points::new(-3.0, -1.0, -4.0));
        assert_eq!(bounds.max, points::new(6.0, 7.0, 2.0));
    }
}