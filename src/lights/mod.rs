use crate::tuples::points::Point;
use crate::tuples::colors::Color;

#[derive(Copy, Clone)]
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
mod tests;