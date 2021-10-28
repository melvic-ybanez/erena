use crate::shapes::{Shape, Space3D};
use crate::rays::{Ray, Intersection3D, Intersection};
use crate::math;
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::tuples::vectors;

pub fn intersect<'a>(cyl: &'a Shape, ray: &Ray) -> Vec<Intersection3D<'a>> {
    let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

    // there are no intersections when ray is parallel to the y axis
    if a <= math::EPSILON {
        return vec![]
    }

    let b = 2.0 * ray.origin.x * ray.direction.x +
        2.0 * ray.origin.z * ray.direction.z;
    let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;

    let disc = b.powi(2) - 4.0 * a * c;

    // no intersections
    if disc < 0.0 {
        return vec![]
    }

    let t0 = (-b - disc.sqrt()) / (2.0 * a);
    let t1 = (-b + disc.sqrt()) / (2.0 * a);

    let (t0, t1) = if t0 > t1 {
        (t1, t0)
    } else {
        (t0, t1)
    };

    let mut xs: Vec<Intersection3D> = vec![];

    if let Space3D::Cylinder(min, max) = cyl.shape {
        let y0 = ray.origin.y + t0 * ray.direction.y;
        if min < y0 && y0 < max {
            xs.push(Intersection::new(t0, &cyl));
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if min < y1 && y1 < max {
            xs.push(Intersection::new(t1, &cyl));
        }
    }

    xs
}

pub fn normal_at(point: Point) -> Vector {
    vectors::new(point.x, 0.0, point.z)
}

#[cfg(test)]
mod tests {
    use crate::shapes::{Shape, Space3D};
    use crate::tuples::{points, vectors};
    use crate::tuples::points::Point;
    use crate::rays::Ray;
    use crate::math;
    use crate::math::Real;
    use crate::shapes::Space3D::Cylinder;

    #[test]
    fn test_ray_misses_cylinder() {
        let cyl = Shape::cylinder();
        let data = [
            (points::new(1.0, 0.0, 0.0), vectors::new(0.0, 1.0, 0.0)),
            (Point::origin(), vectors::new(0.0, 1.0, 0.0)),
            (points::new(0.0, 0.0, -5.0), vectors::new(1.0, 1.0, 1.0))
        ];
        for (origin, direction) in data {
            let direction = direction.normalize();
            let ray = Ray::new(origin, direction);
            let xs = cyl.intersect(&ray);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn test_ray_strikes_cylinder() {
        let cyl = Shape::cylinder();
        let data = [
            (points::new(1.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0), 5.0, 5.0),
            (points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0), 4.0, 6.0),
            (points::new(0.5, 0.0, -5.0), vectors::new(0.1, 1.0, 1.0), 6.80798, 7.08872)
        ];
        for (origin, direction, t0, t1) in data {
            let direction = direction.normalize();
            let ray = Ray::new(origin, direction);
            let xs = cyl.intersect(&ray);
            assert_eq!(xs.len(), 2);
            assert_eq!(math::round(xs[0].t, 5), t0);
            assert_eq!(math::round(xs[1].t, 5), t1);
        }
    }

    #[test]
    fn test_cylinder_normal() {
        let cyl = Shape::cylinder();
        let data = [
            (points::new(1.0, 0.0, 0.0), vectors::new(1.0, 0.0, 0.0)),
            (points::new(0.0, 5.0, -1.0), vectors::new(0.0, 0.0, -1.0)),
            (points::new(0.0, -2.0, 1.0), vectors::new(0.0, 0.0, 1.0)),
            (points::new(-1.0, 1.0, 0.0), vectors::new(-1.0, 0.0, 0.0))
        ];
        for (point, normal) in data {
            let n = cyl.normal_at(point);
            assert_eq!(n, normal);
        }
    }

    /// The default minimum and maximum for a cylinder
    #[test]
    fn test_default_min_max() {
        if let Space3D::Cylinder(min, max) = Shape::cylinder().shape {
            assert_eq!(min, -Real::INFINITY);
            assert_eq!(max, Real::INFINITY);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_intersecting_constrained() {
        let cyl = Shape::cylinder().cyl_min(1.0).cyl_max(2.0);
        let data = [
            (points::new(0.0, 1.5, 0.0), vectors::new(0.1, 1.0, 0.0), 0),
            (points::new(0.0, 3.0, -5.0), vectors::new(0.0, 0.0, 1.0), 0),
            (points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0), 0),
            (points::new(0.0, 2.0, -5.0), vectors::new(0.0, 0.0, 1.0), 0),
            (points::new(0.0, 1.0, -5.0), vectors::new(0.0, 0.0, 1.0), 0),
            (points::new(0.0, 1.5, -2.0), vectors::new(0.0, 0.0, 1.0), 2)
        ];
        for (point, direction, count) in data {
            let direction = direction.normalize();
            let ray = Ray::new(point, direction);
            let xs = cyl.intersect(&ray);
            assert_eq!(xs.len(), count);
        }
    }
}