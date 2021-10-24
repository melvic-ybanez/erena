use crate::lights::PointLight;
use crate::tuples::colors::Color;
use crate::tuples::points::Point;

/// Tests that a point light has a position and intensity
#[test]
fn test_point_light_fields() {
    let intensity = Color::white();
    let position = Point::origin();
    let light = PointLight::new(position, intensity);
    assert_eq!(light.position, position);
    assert_eq!(light.intensity, intensity);
}