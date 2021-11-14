use crate::tuples::colors::Color;
use crate::tuples::points::Point;
use crate::scene::{World, World3D};
use crate::math::Real;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight { position, intensity }
    }

    pub fn intensity_at(&self, point: Point, world: &World3D) -> Real {
        if world.is_shadowed_with_light(self.position, point) {
            0.0
        } else {
            1.0
        }
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