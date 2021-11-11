use crate::shapes::{Shape, Geo};
use crate::matrix::{Matrix, CanTransform, scaling, translation, rotation_y};
use crate::shapes::groups::not_a_group;
use crate::rays::Ray;
use crate::tuples::points::Point;
use crate::tuples::{vectors, points};
use std::rc::Rc;
use crate::math;
use std::borrow::Borrow;

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

    if let Geo::Group(ref g) = group.geo {
        let xs = g.intersect(&group, &ray);
        assert!(xs.is_empty());
    }
}

/// Tests intersection of a ray with a non-empty group.
/// The ray intersects two of the group's children.
#[test]
fn test_intersect_non_empty() {
    let group = Rc::new(Shape::empty_group());
    let s1 = Rc::new(Shape::sphere());
    let s2 = Rc::new(Shape::sphere().translate(0.0, 0.0, -3.0));
    let s3 = Rc::new(Shape::sphere().translate(5.0, 0.0, 0.0));

    if let Geo::Group(g) = &group.geo {
        g.add_child(Rc::downgrade(&group), Rc::clone(&s1));
        g.add_child(Rc::downgrade(&group), Rc::clone(&s2));
        g.add_child(Rc::downgrade(&group), Rc::clone(&s3));

        let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
        let xs = g.intersect(&group, &ray);

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

fn with_nested_object<F>(outer_trans: Matrix, inner_trans: Matrix, child_trans: Matrix, f: F)
    where F: Fn(&Shape) -> () {
    let outer = Rc::new(Shape::empty_group().transform(outer_trans));
    let inner = Rc::new(Shape::empty_group().transform(inner_trans));
    let shape = Rc::new(Shape::sphere().transform(child_trans));

    if let (Geo::Group(g1), Geo::Group(g2)) = (&outer.geo, &inner.geo) {
        g1.add_child(Rc::downgrade(&outer), Rc::clone(&inner));
        g2.add_child(Rc::downgrade(&inner), Rc::clone(&shape));
        f(shape.borrow());
    } else {
        not_a_group();
    }
}

/// Tests converting a point from world to object space
#[test]
fn test_world_to_object_space_conversion() {
    with_nested_object(
        rotation_y(math::PI / 2.0),
        scaling(2.0, 2.0, 2.0),
        translation(5.0, 0.0, 0.0),
        |shape| {
            let point = shape.world_to_object(points::new(-2.0, 0.0, -10.0));
            assert_eq!(point, points::new(0.0, 0.0, -1.0));
        }
    );
}

/// Converting a normal from object to world space
#[test]
fn test_object_normal_to_world_space() {
    with_nested_object(
        rotation_y(math::PI / 2.0),
        scaling(1.0, 2.0, 3.0),
        translation(5.0, 0.0, 0.0),
        |shape| {
            let normal = shape.normal_to_world(
                vectors::new(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0));
            assert_eq!(normal.round_items(), vectors::new(0.28571, 0.42857, -0.85714));
        }
    );
}

/// Find the normal on a child object
#[test]
fn test_normal_on_child() {
    with_nested_object(
        rotation_y(math::PI / 2.0),
        scaling(1.0, 2.0, 3.0),
        translation(5.0, 0.0, 0.0),
        |shape| {
            let normal = shape.default_normal_at(points::new(1.7321, 1.1547, -5.5774));
            assert_eq!(normal.round_items(), vectors::new(0.28570, 0.42854, -0.85716));
        }
    );
}