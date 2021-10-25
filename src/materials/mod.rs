use crate::lights::PointLight;
use crate::math::Real;
use crate::tuples::colors::Color;
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::patterns::Stripe;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: Real,
    pub diffuse: Real,
    pub specular: Real,
    pub shininess: Real,
    pub pattern: Option<Stripe>,
}

impl Material {
    pub fn default() -> Material {
        Material {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
        }
    }

    pub fn lighting(
        &self, light: PointLight,
        point: Point,
        eye_vec: Vector,
        normal_vec: Vector,
        in_shadow: bool,
    ) -> Color {
        let color = match &self.pattern {
            None => self.color,
            Some(pattern) => pattern.at(point)
        };

        // combine the surface color with the light's color
        let effective_color = color * light.intensity;

        // direction of the light source
        let light_vec = (light.position - point).normalize();

        let ambient = effective_color * self.ambient;

        // compute the cosine of the angle between the light vector and the normal vector.
        let light_dot_normal = light_vec.dot(normal_vec);

        // if the cosine is negative, the light is on the other side of the surface
        let (diffuse, specular) = if light_dot_normal < 0.0 || in_shadow {
            (Color::black(), Color::black())
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflect_vec = -light_vec.reflect(normal_vec);

            // compute the cosine of the angle between the reflection vector and the eye vector
            let reflect_dot_eye = reflect_vec.dot(eye_vec);

            // if the cosine is negative, the light reflects away from the eye
            let specular = if reflect_dot_eye <= 0.0 {
                Color::black()
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity * self.specular * factor
            };

            (diffuse, specular)
        };

        ambient + diffuse + specular
    }

    pub fn with_pattern(mut self, pattern: &Stripe) -> Self {
        self.pattern = Some(pattern.clone());
        self
    }
}

#[cfg(test)]
mod tests;