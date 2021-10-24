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
mod tests;