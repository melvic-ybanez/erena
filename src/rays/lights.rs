use crate::math::Real;
use crate::scene::{World3D};
use crate::tuples::colors::Color;
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::tuples::points;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }

    pub fn intensity_at(&self, point: Point, world: &World3D) -> Real {
        if world.is_shadowed_with_light(self.position, point) {
            0.0
        } else {
            1.0
        }
    }
}

pub struct AreaLight {
    pub corner: Point,
    pub u_steps: Step,
    pub v_steps: Step,
    pub intensity: Color,
    u_vec: Vector,
    v_vec: Vector,
    samples: Step,
    position: Point,
}

type Step = usize;

impl AreaLight {
    pub fn new(
        corner: Point,
        full_u_vec: Vector,
        u_steps: Step,
        full_v_vec: Vector,
        v_steps: Step,
        intensity: Color,
    ) -> AreaLight {
        let mid_point = {
            let Vector { x: x1, y: y1, z: z1, .. } = full_u_vec;
            let Vector { x: x2, y: y2, z: z2, .. } = full_v_vec;
            points::new((x1 + x2) / 2.0, (y1 + y2) / 2.0, (z1 + z2) / 2.0)
        };
        AreaLight {
            corner,
            u_steps,
            v_steps,
            u_vec: full_u_vec / u_steps as Real,
            v_vec: full_v_vec / v_steps as Real,
            samples: u_steps * v_steps,
            intensity,
            position: mid_point
        }
    }

    pub fn point_on_light(&self, u: Step, v: Step) -> Point {
        self.corner + self.u_vec * (u as Real + 0.5) + self.v_vec * (v as Real + 0.5)
    }
}

#[cfg(test)]
mod tests {
    use crate::rays::lights::{PointLight, AreaLight};
    use crate::tuples::colors::Color;
    use crate::tuples::points::Point;
    use crate::tuples::{vectors, points};

    /// Tests that a point light has a position and intensity
    #[test]
    fn test_point_light_fields() {
        let intensity = Color::white();
        let position = Point::origin();
        let light = PointLight::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }

    #[test]
    fn test_creating_area_light() {
        let corner = Point::origin();
        let v1 = vectors::new(2.0, 0.0, 0.0);
        let v2 = vectors::new(0.0, 0.0, 1.0);
        let light = AreaLight::new(corner, v1, 4, v2, 2, Color::white());

        assert_eq!(light.corner, corner);
        assert_eq!(light.u_vec, vectors::new(0.5, 0.0, 0.0));
        assert_eq!(light.u_steps, 4);
        assert_eq!(light.v_vec, vectors::new(0.0, 0.0, 0.5));
        assert_eq!(light.v_steps, 2);
        assert_eq!(light.samples, 8);
        assert_eq!(light.position, points::new(1.0, 0.0, 0.5));
    }

    #[test]
    fn test_finding_point_on_area_light() {
        let corner = Point::origin();
        let v1 = vectors::new(2.0, 0.0, 0.0);
        let v2 = vectors::new(0.0, 0.0, 1.0);
        let light = AreaLight::new(corner, v1, 4, v2, 2, Color::white());
        let data = [
            (0, 0, 0.25, 0.0, 0.25),
            (1, 0, 0.75, 0.0, 0.25),
            (0, 1, 0.25, 0.0, 0.75),
            (2, 0, 1.25, 0.0, 0.25),
            (3, 1, 1.75, 0.0, 0.75),
        ];
        for (u, v, x, y, z) in data {
            let point = points::new(x, y, z);
            assert_eq!(light.point_on_light(u, v), point)
        }
    }
}
