use crate::shapes::{Shape, Geometry, cylinders, Group, CylLike};
use crate::tuples::{points, vectors};
use crate::tuples::points::Point;
use crate::rays::Ray;
use crate::math;
use crate::math::Real;
use crate::shapes::Geometry::Cylinder;
use crate::tuples::vectors::Vector;

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
    if let Group::Leaf(Geometry::Cylinder(CylLike { min, max, .. })) = Shape::cylinder().geometry {
        assert_eq!(min, -Real::INFINITY);
        assert_eq!(max, Real::INFINITY);
    } else {
        assert!(false);
    }
}

#[test]
fn test_intersecting_constrained() {
    let cyl = CylLike::cylinder().min(1.0).max(2.0).to_shape();
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

#[test]
fn test_default_closed_value() {
    if let Group::Leaf(Cylinder(CylLike { closed, .. })) = Shape::cylinder().geometry {
        assert!(!closed);
    }
}

#[test]
fn test_intersecting_closed_caps() {
    let cyl = CylLike::cylinder().min(1.0).max(2.0).closed(true).to_shape();
    let data = [
        (points::new(0.0, 3.0, 0.0), vectors::new(0.0, -1.0, 0.0), 2),
        (points::new(0.0, 3.0, -2.0), vectors::new(0.0, -1.0, 2.0), 2),
        (points::new(0.0, 4.0, -2.0), vectors::new(0.0, -1.0, 1.0), 2),
        (points::new(0.0, 0.0, -2.0), vectors::new(0.0, 1.0, 2.0), 2),
        (points::new(0.0, -1.0, -2.0), vectors::new(0.0, 1.0, 1.0), 2)
    ];
    for (point, direction, count) in data {
        let ray = Ray::new(point, direction.normalize());
        let xs = cyl.intersect(&ray);
        assert_eq!(xs.len(), count);
    }
}

/// The normal vector on a cylinder's end caps
#[test]
fn test_cylinder_end_caps_normal() {
    let cyl = CylLike::cylinder().min(1.0).max(2.0).closed(true).to_shape();
    let data = [
        (points::new(0.0, 1.0, 0.0), vectors::new(0.0, -1.0, 0.0)),
        (points::new(0.5, 1.0, 0.0), vectors::new(0.0, -1.0, 0.0)),
        (points::new(0.0, 1.0, 0.5), vectors::new(0.0, -1.0, 0.0)),
        (points::new(0.0, 2.0, 0.0), vectors::new(0.0, 1.0, 0.0)),
        (points::new(0.5, 2.0, 0.0), vectors::new(0.0, 1.0, 0.0)),
        (points::new(0.0, 2.0, 0.5), vectors::new(0.0, 1.0, 0.0))
    ];

    for (point, normal) in data {
        assert_eq!(cyl.normal_at(point), normal);
    }
}

#[test]
fn test_intersecting_a_cone() {
    let shape = Shape::cone();
    let data = [
        (points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0), 5.0, 5.0),
        (points::new(0.0, 0.0, -5.0), vectors::new(1.0, 1.0, 1.0), 8.66025, 8.66025),
        (points::new(1.0, 1.0, -5.0), vectors::new(-0.5, -1.0, 1.0), 4.55006, 49.44994)
    ];

    for (origin, direction, t0, t1) in data {
        let direction = direction.normalize();
        let ray = Ray::new(origin, direction);
        let xs = shape.intersect(&ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(math::round_to_5(xs[0].t), t0);
        assert_eq!(math::round_to_5(xs[1].t), t1);
    }
}

/// Intersecting a cone with a ray parallel to one of its halves
#[test]
fn test_intersecting_cone_parallel_to_half() {
    let shape = Shape::cone();
    let ray = Ray::new(points::new(0.0, 0.0, -1.0), vectors::new(0.0, 1.0, 1.0).normalize());
    let xs = shape.intersect(&ray);
    assert_eq!(xs.len(), 1);
    assert_eq!(math::round_to_5(xs[0].t), 0.35355);
}

#[test]
fn test_normal_on_a_cone() {
    let data = [
        (Point::origin(), Vector::zero()),
        (points::new(1.0, 1.0, 1.0), vectors::new(1.0, -(2_f64.sqrt()), 1.0)),
        (points::new(-1.0, -1.0, 0.0), vectors::new(-1.0, 1.0, 0.0))
    ];
    for (point, normal) in data {
        assert_eq!(cylinders::normal_at(point, -Real::INFINITY, Real::INFINITY, true), normal);
    }
}