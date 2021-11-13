use crate::tuples::points::Point;
use crate::shapes::{Shape, Geo};
use crate::tuples::points;
use crate::math::Real;
use crate::shapes::cylinders::CylLike;
use std::ops::Add;

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
            Geo::TestShape => Bounds::from_min(Point::origin()),
            Geo::Plane => Bounds::from_min(points::new(-Real::INFINITY, 0.0, -Real::INFINITY)),
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

// TODO: Add more bounding box tests here.
#[cfg(test)]
mod tests {
    use crate::tuples::points;
    use crate::shapes::Shape;
    use crate::shapes::bounds::Bounds;
    use crate::math::Real;
    use crate::shapes::cylinders::CylLike;

    #[test]
    fn test_creating_empty_box() {
        let bbox = Bounds::empty();
        let min = points::new(Real::INFINITY, Real::INFINITY, Real::INFINITY);
        let max = (-min).to_point();
        assert_eq!(bbox.min, min);
        assert_eq!(bbox.max, max);
    }

    #[test]
    fn test_adding_points_to_box() {
        let bbox = Bounds::empty();
        let p1 = points::new(-5.0, 2.0, 0.0);
        let p2 = points::new(7.0, 0.0, -3.0);
        let bbox = bbox + p1 + p2;
        assert_eq!(bbox.min, points::new(-5.0, 0.0, -3.0));
        assert_eq!(bbox.max, points::new(7.0, 2.0, 0.0));
    }

    #[test]
    fn test_sphere_bounds() {
        let shape = Shape::sphere();
        let bbox = shape.bounds();
        assert_eq!(bbox.min, points::new(-1.0, -1.0, -1.0));
        assert_eq!(bbox.max, points::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_plane_bounds() {
        let bbox = Shape::plane().bounds();
        assert_eq!(bbox.min, points::new(-Real::INFINITY, 0.0, -Real::INFINITY));
        assert_eq!(bbox.max, points::new(Real::INFINITY, 0.0, Real::INFINITY));
    }

    #[test]
    fn test_cube_bounds() {
        let bbox = Shape::cube().bounds();
        assert_eq!(bbox.min, points::new(-1.0, -1.0, -1.0));
        assert_eq!(bbox.max, points::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_unbounded_cylinder_bounds() {
        let bbox = Shape::cylinder().bounds();
        assert_eq!(bbox.min, points::new(-1.0, -Real::INFINITY, -1.0));
        assert_eq!(bbox.max, points::new(1.0, Real::INFINITY, 1.0));
    }

    #[test]
    fn test_bounded_cylinder_bounds() {
        let bbox = CylLike::cylinder()
            .min(-5.0).max(3.0)
            .to_shape()
            .bounds();
        assert_eq!(bbox.min, points::new(-1.0, -5.0, -1.0));
        assert_eq!(bbox.max, points::new(1.0, 3.0, 1.0));
    }

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