use crate::shapes::{Shape, Geometry, Group };
use crate::rays::{Ray, Intersection3D, Intersection};
use crate::math;
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::tuples::vectors;
use crate::math::{Real, EPSILON};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct CylLike {
    pub min: Real,
    pub max: Real,
    pub closed: bool,
    pub cone: bool,     // consider defining cone as its own type?
}

impl CylLike {
    pub fn new(cone: bool) -> CylLike {
        CylLike {
            min: -Real::INFINITY,
            max: Real::INFINITY,
            closed: false,
            cone,
        }
    }

    pub fn cylinder() -> CylLike {
        CylLike::new(false)
    }

    pub fn cone() -> CylLike {
        CylLike::new(true)
    }

    pub fn min(mut self, min: Real) -> CylLike {
        self.min = min;
        self
    }

    pub fn max(mut self, max: Real) -> Self {
        self.max = max;
        self
    }

    pub fn closed(mut self, closed: bool) -> Self {
        self.closed = closed;
        self
    }

    pub fn is_cone(&self) -> bool {
        self.cone
    }

    pub fn to_geo(&self) -> Geometry {
        Geometry::Cylinder(self.clone())
    }

    pub fn to_shape(&self) -> Shape {
        Shape::one(self.to_geo())
    }
}

pub fn intersect<'a>(cyl: &'a Shape, ray: &Ray, cone: bool) -> Vec<Intersection3D<'a>> {
    let Ray { origin: o, direction: d } = ray;

    let dx2 = d.x.powi(2);
    let dy2 = d.y.powi(2);
    let dz2 = d.z.powi(2);

    let a = dx2 + dz2;
    let b = 2.0 * o.x * d.x + 2.0 * o.z * d.z;
    let c = o.x.powi(2) + o.z.powi(2) - 1.0;

    let (a, b, c) = if cone {
        let a = dx2 - dy2 + dz2;
        let b = b - 2.0 * o.y * d.y;
        let c = o.x.powi(2) - o.y.powi(2) + o.z.powi(2);

        if math::compare_reals(a, 0.0) && !math::compare_reals(b, 0.0) {
            let t = -c / (2.0 * b);
            return vec![Intersection::new(t, cyl)];
        } else if math::compare_reals(a, 0.0) {
            return vec![];
        }

        (a, b, c)
    } else {
        (a, b, c)
    };

    // there are no intersections when ray is parallel to the y axis
    if math::compare_reals(a, 0.0) {
        return intersect_caps(cyl, ray, vec![]);
    }

    let disc = b.powi(2) - 4.0 * a * c;

    // no intersections
    if disc < 0.0 {
        return vec![];
    }

    let t0 = (-b - disc.sqrt()) / (2.0 * a);
    let t1 = (-b + disc.sqrt()) / (2.0 * a);

    let (t0, t1) = if t0 > t1 {
        (t1, t0)
    } else {
        (t0, t1)
    };

    let mut xs: Vec<Intersection3D> = vec![];

    if let Group::Leaf(Geometry::Cylinder(CylLike { min, max, .. })) = cyl.geometry {
        let mut y_between_t = |t: Real| {
            let y = o.y + t * d.y;
            if min < y && y < max {
                xs.push(Intersection::new(t, cyl));
            }
        };

        y_between_t(t0);
        y_between_t(t1);
    }

    intersect_caps(cyl, ray, xs)
}

pub fn normal_at(point: Point, min: Real, max: Real, cone: bool) -> Vector {
    // square of the distance from y-axis
    let dist = point.x.powi(2) + point.z.powi(2);

    if dist < 1.0 && point.y >= max - math::EPSILON {
        vectors::new(0.0, 1.0, 0.0)
    } else if dist < 1.0 && point.y <= min + EPSILON {
        vectors::new(0.0, -1.0, 0.0)
    } else {
        let y = if cone {
            let y = (point.x.powi(2) + point.z.powi(2)).sqrt();
            if point.y > 0.0 { -y } else { y }
        } else {
            0.0
        };
        vectors::new(point.x, y, point.z)
    }
}

/// Checks if the intersection is within the radius. If it is,
/// include the intersection
fn check_cap<'a>(cyl: &'a Shape, ray: &Ray, limit: Real, xs: &mut Vec<Intersection3D<'a>>) {
    let t = (limit - ray.origin.y) / ray.direction.y;
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;

    let radius = if cyl.geometry.is_cone() { limit.abs() } else { 1.0 };
    if (x * x + z * z) <= radius {
        xs.push(Intersection::new(t, cyl));
    }
}

fn intersect_caps<'a>(cyl: &'a Shape, ray: &Ray, mut xs: Vec<Intersection3D<'a>>) -> Vec<Intersection3D<'a>> {
    if let Group::Leaf(Geometry::Cylinder(CylLike { min, max, closed, .. })) = cyl.geometry {
        // not closed or no intersection. Reject.
        if !closed || math::compare_reals(ray.direction.y, 0.0) {
            return xs
        }

        check_cap(cyl, ray, min, &mut xs);
        check_cap(cyl, ray, max, &mut xs);
        return xs
    }
    xs
}