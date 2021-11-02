use crate::shapes::{Shape, Geo, groups};
use crate::matrix::{Matrix, CanTransform};
use crate::shapes::groups::not_a_group;
use crate::rays::Ray;
use crate::tuples::points::Point;
use crate::tuples::{vectors, points};
use std::rc::Rc;

#[test]
fn test_create_group() {
    let group = Shape::empty_group();
    assert_eq!(group.transformation, Matrix::id44());
    if let Geo::Group(group) = group.geo {
        assert!(group.is_empty());
    } else {
        not_a_group();
    }
}

#[test]
fn test_shape_parent() {
    let shape = Shape::test();
    assert_eq!(shape.parent.borrow().weak_count(), 0);
}

#[test]
fn test_add_child() {
    let group = Rc::new(Shape::empty_group());
    let shape = Rc::new(Shape::test());

    if let Geo::Group(g) = &group.geo {
        g.add_child(Rc::downgrade(&group), Rc::clone(&shape));
        assert!(g.non_empty());
        assert!(g.contains(shape));
    } else {
        not_a_group();
    }
}

/// Tests intersection of a ray with an empty group
#[test]
fn test_intersect_empty() {
    let group = Shape::empty_group();
    let ray = Ray::new(Point::origin(), vectors::new(0.0, 0.0, 1.0));
    let xs = groups::intersect(&group, &ray);
    assert!(xs.is_empty());
}

/// Tests intersection of a ray with a non-empty group.
/// The ray intersects two of the group's children.
#[test]
fn test_intersect_non_empty() {
    let mut group = Rc::new(Shape::empty_group());
    let mut s1 = Rc::new(Shape::sphere());
    let mut s2 = Rc::new(Shape::sphere().translate(0.0, 0.0, -3.0));
    let mut s3 = Rc::new(Shape::sphere().translate(5.0, 0.0, 0.0));

    if let Geo::Group(g) = &group.geo {
        g.add_child(Rc::downgrade(&group), Rc::clone(&s1));
        g.add_child(Rc::downgrade(&group), Rc::clone(&s2));
        g.add_child(Rc::downgrade(&group), Rc::clone(&s3));

        let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
        let xs = groups::intersect(&group, &ray);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object, s2);
        assert_eq!(xs[1].object, s2);
        assert_eq!(xs[2].object, s1);
        assert_eq!(xs[3].object, s1);
    } else {
        not_a_group();
    }
}

/// Tests intersecting a transformed group
#[test]
fn test_intersect_transformed() {
    let group = Rc::new(Shape::empty_group().scale(2.0, 2.0, 2.0));
    let shape = Rc::new(Shape::sphere().translate(5.0, 0.0, 0.0));

    if let Geo::Group(g) = &group.geo {
        g.add_child(Rc::downgrade(&group), shape);
        let ray = Ray::new(points::new(10.0, 0.0, -10.0), vectors::new(0.0, 0.0, 1.0));
        let xs = group.intersect(&ray);
        assert_eq!(xs.len(), 2);
    } else {
        not_a_group();
    }
}