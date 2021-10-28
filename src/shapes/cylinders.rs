use crate::shapes::Shape;
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

    vec![Intersection::new(t0, cyl), Intersection::new(t1, cyl)]
}

pub fn normal_at(point: Point) -> Vector {
    vectors::new(point.x, 0.0, point.z)
}

#[cfg(test)]
mod tests {
    use crate::shapes::Shape;
    use crate::tuples::{points, vectors};
    use crate::tuples::points::Point;
    use crate::rays::Ray;
    use crate::math;

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
}