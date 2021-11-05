use crate::shapes::Shape;
use crate::rays::{Ray, Intersection3D, Intersection};
use crate::tuples::vectors::Vector;
use crate::tuples::points::Point;
use crate::math::Real;
use crate::math;
use crate::tuples::vectors;
use std::rc::Rc;
use crate::shapes::bounds::Bounds;

pub fn intersect(cube: &Shape, ray: &Ray) -> Vec<Intersection3D> {
    let Ray { origin, direction } = ray;
    let Bounds { min, max } = cube.bounds();
    let (x_t_min, x_t_max) = check_axis(origin.x, direction.x, min.x, max.x);
    let (y_t_min, y_t_max) = check_axis(origin.y, direction.y, min.y, max.y);
    let (z_t_min, z_t_max) = check_axis(origin.z, direction.z, min.z, max.z);

    let t_min = Real::max(x_t_min, Real::max(y_t_min, z_t_min));
    let t_max = Real::min(x_t_max, Real::min(y_t_max, z_t_max));

    if t_min > t_max {
        vec![]
    } else {
        vec![
            Intersection::new(t_min, Rc::new(cube.clone())),
            Intersection::new(t_max, Rc::new(cube.clone()))
        ]
    }
}

pub fn normal_at(point: Point) -> Vector {
    let max = Real::max(point.x.abs(), Real::max(point.y.abs(), point.z.abs()));
    if max == point.x.abs() {
        vectors::new(point.x, 0.0, 0.0)
    } else if max == point.y.abs() {
        vectors::new(0.0, point.y, 0.0)
    } else {
        vectors::new(0.0, 0.0, point.z)
    }
}

fn check_axis(origin: Real, direction: Real, min: Real, max: Real) -> (Real, Real) {
    let t_min_num = min - origin;
    let t_max_num = max - origin;

    let (t_min, t_max) = if direction.abs() >= math::EPSILON {
        (t_min_num / direction, t_max_num / direction)
    } else {
        (t_min_num * Real::INFINITY, t_max_num * Real::INFINITY)
    };

    if t_min > t_max {
        (t_max, t_min)
    } else {
        (t_min, t_max)
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::Shape;
    use crate::rays::Ray;
    use crate::tuples::{points, vectors};

    #[test]
    fn test_ray_cube_intersection() {
        let cube = Shape::cube();
        let data = [
            (points::new(5.0, 0.5, 0.0), vectors::new(-1.0, 0.0, 0.0), 4.0, 6.0),   // +x
            (points::new(-5.0, 0.5, 0.0), vectors::new(1.0, 0.0, 0.0), 4.0, 6.0),   // -x
            (points::new(0.5, 5.0, 0.0), vectors::new(0.0, -1.0, 0.0), 4.0, 6.0),   // +y
            (points::new(0.5, -5.0, 0.0), vectors::new(0.0, 1.0, 0.0), 4.0, 6.0),   // -y
            (points::new(0.5, 0.0, 5.0), vectors::new(0.0, 0.0, -1.0), 4.0, 6.0),   // +z
            (points::new(0.5, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0), 4.0, 6.0),   // -z
            (points::new(0.0, 0.5, 0.0), vectors::new(0.0, 0.0, 1.0), -1.0, 1.0),   // inside
        ];
        for (origin, direction, t1, t2) in data.iter() {
            let ray = Ray::new(*origin, *direction);
            let xs = cube.intersect(&ray);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, *t1);
            assert_eq!(xs[1].t, *t2);
        }
    }

    #[test]
    fn test_ray_misses_a_cube() {
        let cube = Shape::cube();
        let data = [
            (points::new(-2.0, 0.0, 0.0), vectors::new(0.2673, 0.5345, 0.8018)),
            (points::new(0.0, -2.0, 0.0), vectors::new(0.8018, 0.2673, 0.5345)),
            (points::new(0.0, 0.0, -2.0), vectors::new(0.5345, 0.8018, 0.2673)),
            (points::new(2.0, 0.0, 2.0), vectors::new(0.0, 0.0, -1.0)),
            (points::new(0.0, 2.0, 2.0), vectors::new(0.0, -1.0, 0.0)),
            (points::new(2.0, 2.0, 0.0), vectors::new(-1.0, 0.0, 0.0)),
        ];
        for (origin, direction) in data.iter() {
            let ray = Ray::new(*origin, *direction);
            let xs = cube.intersect(&ray);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn test_cube_normal() {
        let cube = Shape::cube();
        let data = [
            (points::new(1.0, 0.5, -0.8), vectors::new(1.0, 0.0, 0.0)),
            (points::new(-1.0, -0.2, 0.9), vectors::new(-1.0, 0.0, 0.0)),
            (points::new(-0.4, 1.0, -0.1), vectors::new(0.0, 1.0, 0.0)),
            (points::new(0.3, -1.0, -0.7), vectors::new(0.0, -1.0, 0.0)),
            (points::new(-0.6, 0.3, 1.0), vectors::new(0.0, 0.0, 1.0)),
            (points::new(0.4, 0.4, -1.0), vectors::new(0.0, 0.0, -1.0)),
            (points::new(1.0, 1.0, 1.0), vectors::new(1.0, 0.0, 0.0)),
            (points::new(-1.0, -1.0, -1.0), vectors::new(-1.0, 0.0, 0.0))
        ];
        for (point, normal) in data {
            assert_eq!(cube.normal_at(point), normal);
        }
    }
}