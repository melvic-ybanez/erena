use crate::shapes::{Geo, Shape};
use crate::shapes::triangles::Triangle;
use crate::tuples::{points, vectors};

#[test]
fn test_constructing_triangle() {
    let p1 = points::new(0.0, 1.0, 0.0);
    let p2 = points::new(-1.0, 0.0, 0.0);
    let p3 = points::new(1.0, 0.0, 0.0);
    let triangle = Shape::triangle(p1, p2, p3);

    if let Geo::Triangle(t) = triangle.geo {
        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.get_edge1(), vectors::new(-1.0, -1.0, 0.0));
        assert_eq!(t.get_edge2(), vectors::new(1.0, -1.0, 0.0));
        assert_eq!(t.get_normal(), vectors::new(0.0, 0.0, -1.0));
    } else {
        panic!("Not a triangle");
    }
}

#[test]
fn test_triangle_normal() {
    let triangle = Shape::triangle(
        points::new(0.0, 1.0, 0.0),
        points::new(-1.0, 0.0, 0.0),
        points::new(1.0, 0.0, 0.0)
    );

    if let Geo::Triangle(ref tri) = triangle.geo {
        assert_eq!(triangle.normal_at(points::new(0.0, 0.5, 0.0)), tri.get_normal());
        assert_eq!(triangle.normal_at(points::new(-0.5, 0.75, 0.0)), tri.get_normal());
        assert_eq!(triangle.normal_at(points::new(0.5, 0.25, 0.0)), tri.get_normal());
    }
}