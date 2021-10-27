use crate::tuples::colors::Color;
use crate::tuples::points::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight { position, intensity }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::colors::Color;
    use crate::tuples::points::Point;
    use crate::rays::lights::PointLight;

    /// Tests that a point light has a position and intensity
    #[test]
    fn test_point_light_fields() {
        let intensity = Color::white();
        let position = Point::origin();
        let light = PointLight::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}