use crate::patterns::{Pattern, PatternType};
use crate::tuples::colors::Color;
use crate::tuples::{points, colors};
use crate::tuples::points::Point;
use crate::shapes::Shape;
use crate::matrix::CanTransform;

fn default_stripe() -> Pattern {
    Pattern::stripe(Color::white(), Color::black())
}

#[test]
fn test_creating_stripe() {
    let pattern = default_stripe();
    if let PatternType::Stripe(first, second) = pattern.pattern_type {
        assert_eq!(first, Color::white());
        assert_eq!(second, Color::black());
    } else {
        assert!(false);
    }
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

/// A gradient linearly interpolates between colors
#[test]
fn test_gradient_linear_interpolation() {
    let pattern = Pattern::gradient(Color::white(), Color::black());
    assert_eq!(pattern.at(Point::origin()), Color::white());
    assert_eq!(pattern.at(points::new(0.25, 0.0, 0.0)), colors::new(0.75, 0.75, 0.75));
    assert_eq!(pattern.at(points::new(0.5, 0.0, 0.0)), colors::new(0.5, 0.5, 0.5));
    assert_eq!(pattern.at(points::new(0.75, 0.0, 0.0)), colors::new(0.25, 0.25, 0.25));
}