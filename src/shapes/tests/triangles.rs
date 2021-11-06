use crate::shapes::{Geo, Shape};
use crate::tuples::{points, vectors};
use crate::rays::Ray;

fn set_up() -> Shape {
    Shape::triangle(
        points::new(0.0, 1.0, 0.0),
        points::new(-1.0, 0.0, 0.0),
        points::new(1.0, 0.0, 0.0)
    )
}

#[test]
fn test_constructing_triangle() {
    let p1 = points::new(0.0, 1.0, 0.0);
    let p2 = points::new(-1.0, 0.0, 0.0);
    let p3 = points::new(1.0, 0.0, 0.0);
    let triangle = Shape::triangle(p1, p2, p3);

    if let Geo::Triangle(t) = triangle.geo {
        assert_eq!(t.get_p1(), p1);
        assert_eq!(t.get_p2(), p2);
        assert_eq!(t.get_p3(), p3);
        assert_eq!(t.get_edge1(), vectors::new(-1.0, -1.0, 0.0));
        assert_eq!(t.get_edge2(), vectors::new(1.0, -1.0, 0.0));
        assert_eq!(t.get_normal(), vectors::new(0.0, 0.0, -1.0));
    } else {
        panic!("Not a triangle");
    }
}

#[test]
fn test_triangle_normal() {
    let triangle = set_up();

    if let Geo::Triangle(ref tri) = triangle.geo {
        assert_eq!(triangle.normal_at(points::new(0.0, 0.5, 0.0)), tri.get_normal());
        assert_eq!(triangle.normal_at(points::new(-0.5, 0.75, 0.0)), tri.get_normal());
        assert_eq!(triangle.normal_at(points::new(0.5, 0.25, 0.0)), tri.get_normal());
    }
}

/// Tests intersection with a ray parallel to the surface
/// of the triangle.
#[test]
fn test_intersect_parallel() {
    let triangle = set_up();
    let ray = Ray::new(points::new(0.0, -1.0, -2.0), vectors::new(0.0, 1.0, 0.0));
    assert!(triangle.intersect(&ray).is_empty());
}

#[test]
fn test_ray_misses_p1_p3_edge() {
    let triangle = set_up();
    let ray = Ray::new(points::new(1.0, 1.0, -2.0), vectors::new(0.0, 0.0, 1.0));
    let xs = triangle.intersect(&ray);
    assert!(xs.is_empty());
}

#[test]
fn test_ray_misses_p1_p2_edge() {
    let triangle = set_up();
    let ray = Ray::new(points::new(-1.0, 1.0, -2.0), vectors::new(0.0, 0.0, 1.0));
    assert!(triangle.intersect(&ray).is_empty());
}

#[test]
fn test_ray_misses_p2_p3_edge() {
    let triangle = set_up();
    let ray = Ray::new(points::new(0.0, -1.0, -2.0), vectors::new(0.0, 0.0, 1.0));
    assert!(triangle.intersect(&ray).is_empty());
}

#[test]
fn test_ray_strikes_triangle() {
    let triangle = set_up();
    let ray = Ray::new(points::new(0.0, 0.5, -2.0), vectors::new(0.0, 0.0, 1.0));
    let xs = triangle.intersect(&ray);
    assert_eq!(xs.len(), 1);
    assert_eq!(xs[0].t, 2.0);
}