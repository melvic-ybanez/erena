use crate::math;
use crate::rays::{Comps, Intersection, IntersectionKind, Ray};
use crate::shapes::triangles::{Smooth, TriangleKind};
use crate::shapes::{Geo, Shape};
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::tuples::{points, vectors};
use std::rc::Rc;

fn tri_points() -> (Point, Point, Point) {
    (
        points::new(0.0, 1.0, 0.0),
        points::new(-1.0, 0.0, 0.0),
        points::new(1.0, 0.0, 0.0),
    )
}

fn smooth_tri_normals() -> (Vector, Vector, Vector) {
    (
        vectors::new(0.0, 1.0, 0.0),
        vectors::new(-1.0, 0.0, 0.0),
        vectors::new(1.0, 0.0, 0.0),
    )
}

fn triangle() -> Shape {
    let (p1, p2, p3) = tri_points();
    Shape::triangle(p1, p2, p3)
}

fn smooth_triangle() -> Shape {
    let (p1, p2, p3) = tri_points();
    let (n1, n2, n3) = smooth_tri_normals();
    Shape::smooth_triangle(p1, p2, p3, n1, n2, n3)
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
        assert_eq!(t.get_default_normal(), vectors::new(0.0, 0.0, -1.0));
    } else {
        panic!("Not a triangle");
    }
}

#[test]
fn test_triangle_normal() {
    let triangle = triangle();

    if let Geo::Triangle(ref tri) = triangle.geo {
        assert_eq!(
            triangle.default_normal_at(points::new(0.0, 0.5, 0.0)),
            tri.get_default_normal()
        );
        assert_eq!(
            triangle.default_normal_at(points::new(-0.5, 0.75, 0.0)),
            tri.get_default_normal()
        );
        assert_eq!(
            triangle.default_normal_at(points::new(0.5, 0.25, 0.0)),
            tri.get_default_normal()
        );
    }
}

/// Tests intersection with a ray parallel to the surface
/// of the triangle.
#[test]
fn test_intersect_parallel() {
    let triangle = triangle();
    let ray = Ray::new(points::new(0.0, -1.0, -2.0), vectors::new(0.0, 1.0, 0.0));
    assert!(triangle.intersect(&ray).is_empty());
}

#[test]
fn test_ray_misses_p1_p3_edge() {
    let triangle = triangle();
    let ray = Ray::new(points::new(1.0, 1.0, -2.0), vectors::new(0.0, 0.0, 1.0));
    let xs = triangle.intersect(&ray);
    assert!(xs.is_empty());
}

#[test]
fn test_ray_misses_p1_p2_edge() {
    let triangle = triangle();
    let ray = Ray::new(points::new(-1.0, 1.0, -2.0), vectors::new(0.0, 0.0, 1.0));
    assert!(triangle.intersect(&ray).is_empty());
}

#[test]
fn test_ray_misses_p2_p3_edge() {
    let triangle = triangle();
    let ray = Ray::new(points::new(0.0, -1.0, -2.0), vectors::new(0.0, 0.0, 1.0));
    assert!(triangle.intersect(&ray).is_empty());
}

#[test]
fn test_ray_strikes_triangle() {
    let triangle = triangle();
    let ray = Ray::new(points::new(0.0, 0.5, -2.0), vectors::new(0.0, 0.0, 1.0));
    let xs = triangle.intersect(&ray);
    assert_eq!(xs.len(), 1);
    assert_eq!(xs[0].t, 2.0);
}

#[test]
fn test_construct_smooth_triangle() {
    let triangle = smooth_triangle();
    let (p1, p2, p3) = tri_points();
    let (n1, n2, n3) = smooth_tri_normals();
    if let Geo::Triangle(tri) = triangle.geo {
        assert_eq!(tri.get_p1(), p1);
        assert_eq!(tri.get_p2(), p2);
        assert_eq!(tri.get_p3(), p3);

        if let TriangleKind::Smooth(Smooth {
            n1: sn1,
            n2: sn2,
            n3: sn3,
        }) = tri.kind
        {
            assert_eq!(sn1, n1);
            assert_eq!(sn2, n2);
            assert_eq!(sn3, n3);
        } else {
            panic!("Not a smooth triangle");
        }
    } else {
        panic!("Not a smooth triangle");
    }
}

/// Checks that an intersection with a smooth triangle
/// preserves the u and v properties
#[test]
fn test_intersection_stores_uv() {
    let tri = smooth_triangle();
    let ray = Ray::new(points::new(-0.2, 0.3, -2.0), vectors::new(0.0, 0.0, 1.0));
    let xs = tri.intersect(&ray);

    for intersection in xs.iter() {
        if let IntersectionKind::Triangle { u, v } = intersection.get_kind() {
            assert_eq!(math::round(u, 2), 0.45);
            assert_eq!(math::round(v, 2), 0.25);
        } else {
            panic!("Intersection kind is not triangle");
        }
    }
}

/// A smooth triangle uses u and v to interpolate the normal
#[test]
fn test_uv_interpolation_with_normal() {
    let tri = Rc::new(smooth_triangle());
    let i = Intersection::new_with_uv(1.0, Rc::clone(&tri), 0.45, 0.25);
    let n = tri.normal_at(Point::origin(), &i);
    assert_eq!(n, vectors::new(-0.5547, 0.83205, 0.0));
}

/// Preparing the normal on a smooth triangle
#[test]
fn test_prepare_normal_for_smooth() {
    let tri = smooth_triangle();
    let intersection = Intersection::new_with_uv(1.0, Rc::new(tri), 0.45, 0.25);
    let ray = Ray::new(points::new(-0.2, 0.3, -2.0), vectors::new(0.0, 0.0, 1.0));
    let xs = vec![intersection.clone()];
    let comps = Comps::prepare(&intersection, &ray, &xs);
    assert_eq!(comps.get_normal_vec(), vectors::new(-0.5547, 0.83205, 0.0));
}
