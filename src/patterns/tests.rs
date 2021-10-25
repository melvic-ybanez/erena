use crate::patterns::Stripe;
use crate::tuples::colors::Color;
use crate::tuples::points;
use crate::tuples::points::Point;

#[test]
fn test_creating_stripe() {
    let pattern = Stripe::new(Color::white(), Color::black());
    assert_eq!(pattern.0, Color::white());
    assert_eq!(pattern.1, Color::black());
}

/// A stripe pattern is constant in y
#[test]
fn test_stripe_in_y() {
    let pattern = Stripe::new(Color::white(), Color::black());
    assert_eq!(pattern.at(Point::origin()), Color::white());
    assert_eq!(pattern.at(points::new(0.0, 1.0, 0.0)), Color::white());
    assert_eq!(pattern.at(points::new(0.0, 2.0, 0.0)), Color::white());
}

/// A stripe pattern is constant in z
#[test]
fn test_stripe_in_z() {
    let pattern = Stripe::new(Color::white(), Color::black());
    assert_eq!(pattern.at(Point::origin()), Color::white());
    assert_eq!(pattern.at(points::new(0.0, 0.0, 1.0)), Color::white());
    assert_eq!(pattern.at(points::new(0.0, 0.0, 2.0)), Color::white());
}

/// A stripe pattern is constant in x
#[test]
fn test_stripe_in_x() {
    let pattern = Stripe::new(Color::white(), Color::black());
    assert_eq!(pattern.at(Point::origin()), Color::white());
    assert_eq!(pattern.at(points::new(0.9, 0.0, 0.0)), Color::white());
    assert_eq!(pattern.at(points::new(1.0, 0.0, 0.0)), Color::black());
    assert_eq!(pattern.at(points::new(-0.1, 0.0, 0.0)), Color::black());
    assert_eq!(pattern.at(points::new(-1.0, 0.0, 0.0)), Color::black());
    assert_eq!(pattern.at(points::new(-1.1, 0.0, 0.0)), Color::white());
}