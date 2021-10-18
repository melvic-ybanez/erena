use crate::tuples::colors::Color;
use crate::math::Real;

pub struct Material {
    pub color: Color,
    pub ambient: Real,
    pub diffuse: Real,
    pub specular: Real,
    pub shininess: Real,
}

impl Material {
    pub fn default() -> Material {
        Material {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn new(color: Color, ambient: Real, diffuse: Real, specular: Real, shininess: Real) -> Material {
        Material { color, ambient, diffuse, specular, shininess }
    }
}

#[cfg(test)]
mod tests;