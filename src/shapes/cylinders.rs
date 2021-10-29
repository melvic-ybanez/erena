use crate::shapes::{Shape, Space3D};
use crate::rays::{Ray, Intersection3D, Intersection};
use crate::math;
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::tuples::vectors;
use crate::math::{Real, EPSILON};

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

    if let Space3D::Cylinder { min, max, .. } = cyl.shape {
        let y0 = ray.origin.y + t0 * ray.direction.y;
        if min < y0 && y0 < max {
            xs.push(Intersection::new(t0, &cyl));
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if min < y1 && y1 < max {
            xs.push(Intersection::new(t1, &cyl));
        }
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

    let radius = if cyl.shape.is_cone() { limit.abs() } else { 1.0 };
    if (x * x + z * z) <= radius {
        xs.push(Intersection::new(t, cyl));
    }
}

fn intersect_caps<'a>(cyl: &'a Shape, ray: &Ray, mut xs: Vec<Intersection3D<'a>>) -> Vec<Intersection3D<'a>> {
    if let Space3D::Cylinder { min, max, closed, .. } = cyl.shape {
        // not closed or no intersection. Reject.
        if !closed || math::compare_reals(ray.direction.y, 0.0) {
            return xs;
        }

        check_cap(cyl, ray, min, &mut xs);
        check_cap(cyl, ray, max, &mut xs);
        return xs
    }
    xs
}