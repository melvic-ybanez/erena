use crate::shapes::{Shape, planes, Space3D};
use crate::tuples::points::Point;
use crate::tuples::{points, vectors};

/// The normal of a plane is the same everywhere
#[test]
fn test_normal_at() {
    let plane = Shape::plane();
    let n1 = plane.normal_at(Point::origin());
    let n2 = plane.normal_at(points::new(10.0, 0.0, -10.0));
    let n3 = plane.normal_at(points::new(-5.0, 0.0, 150.0));

    assert_eq!(plane.shape, Space3D::Plane);
    assert_eq!(n1, vectors::new(0.0, 1.0, 0.0));
    assert_eq!(n2, vectors::new(0.0, 1.0, 0.0));
    assert_eq!(n3, vectors::new(0.0, 1.0, 0.0));
}