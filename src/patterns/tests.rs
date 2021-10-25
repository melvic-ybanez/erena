use crate::patterns::Stripe;
use crate::tuples::colors::Color;
use crate::tuples::points;
use crate::tuples::points::Point;
use crate::shapes::Shape;
use crate::matrix::CanTransform;

fn default_stripe() -> Stripe {
    Stripe::new(Color::white(), Color::black())
}

#[test]
fn test_creating_stripe() {
    let pattern = default_stripe();
    assert_eq!(pattern.0, Color::white());
    assert_eq!(pattern.1, Color::black());
}

/// A stripe pattern is constant in y
#[test]
fn test_stripe_in_y() {
    let pattern = default_stripe();
    assert_eq!(pattern.at(Point::origin()), Color::white());
    assert_eq!(pattern.at(points::new(0.0, 1.0, 0.0)), Color::white());
    assert_eq!(pattern.at(points::new(0.0, 2.0, 0.0)), Color::white());
}

/// A stripe pattern is constant in z
#[test]
fn test_stripe_in_z() {
    let pattern = default_stripe();
    assert_eq!(pattern.at(Point::origin()), Color::white());
    assert_eq!(pattern.at(points::new(0.0, 0.0, 1.0)), Color::white());
    assert_eq!(pattern.at(points::new(0.0, 0.0, 2.0)), Color::white());
}

/// A stripe pattern is constant in x
#[test]
fn test_stripe_in_x() {
    let pattern = default_stripe();
    assert_eq!(pattern.at(Point::origin()), Color::white());
    assert_eq!(pattern.at(points::new(0.9, 0.0, 0.0)), Color::white());
    assert_eq!(pattern.at(points::new(1.0, 0.0, 0.0)), Color::black());
    assert_eq!(pattern.at(points::new(-0.1, 0.0, 0.0)), Color::black());
    assert_eq!(pattern.at(points::new(-1.0, 0.0, 0.0)), Color::black());
    assert_eq!(pattern.at(points::new(-1.1, 0.0, 0.0)), Color::white());
}

#[test]
fn test_stripes_with_object_transformation() {
    let object = Shape::sphere().scale(2.0, 2.0, 2.0);
    let pattern = default_stripe().translate(0.5, 0.0, 0.0);
    let c = pattern.at_object(&object, points::new(1.5, 0.0, 0.0));
    assert_eq!(c, Color::white());
}

#[test]
fn test_stripes_with_pattern_transformation() {
    let object = Shape::sphere();
    let pattern = default_stripe().scale(2.0, 2.0, 2.0);
    let c = pattern.at_object(&object, points::new(1.5, 0.0, 0.0));
    assert_eq!(c, Color::white());
}

/// Stripes with both an object and pattern transformation
#[test]
fn test_stripes_with_both_transformation() {
    let object = Shape::sphere().scale(2.0, 0.0, 0.0);
    let pattern = default_stripe().translate(0.5, 0.0, 0.0);
    let c = pattern.at_object(&object, points::new(2.5, 0.0, 0.0));
    assert_eq!(c, Color::white());
}