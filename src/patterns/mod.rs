use crate::tuples::colors::Color;
use crate::tuples::points::Point;
use crate::matrix::{CanTransform, Matrix};
use crate::shapes::Object;
use crate::math::Real;
use crate::tuples::colors;

#[derive(Clone, PartialEq, Debug)]
pub struct Pattern {
    pub pattern_type: PatternType,
    pub transformation: Matrix,
    first: Color,
    second: Color,
}

#[derive(Clone, PartialEq, Debug)]
pub enum PatternType {
    Stripe,
    Gradient,
    Ring,
    Checkers,
    Test,
}

impl Pattern {
    fn new(pattern_type: PatternType, first: Color, second: Color) -> Pattern {
        Pattern { pattern_type, first, second, transformation: Matrix::id44() }
    }

    pub fn stripe(first: Color, second: Color) -> Pattern {
        Pattern::new(PatternType::Stripe, first, second)
    }

    pub fn gradient(first: Color, second: Color) -> Pattern {
        Pattern::new(PatternType::Gradient, first, second)
    }

    pub fn ring(first: Color, second: Color) -> Pattern {
        Pattern::new(PatternType::Ring, first, second)
    }

    pub fn checkers(first: Color, second: Color) -> Pattern {
        Pattern::new(PatternType::Checkers, first, second)
    }

    pub fn test() -> Pattern {
        Pattern::new(PatternType::Test, Color::white(), Color::black())
    }

    pub fn at(&self, point: Point) -> Color {
        let choose = |value: Real| {
            if value % 2.0 == 0.0 {
                self.first
            } else {
                self.second
            }
        };

        match self.pattern_type {
            PatternType::Stripe => choose(point.x.floor()),
            PatternType::Gradient => {
                let distance = self.second - self.first;
                let fraction = point.x - point.x.floor();
                self.first + distance * fraction
            },
            PatternType::Ring => choose((point.x.powi(2) + point.z.powi(2)).sqrt().floor()),
            PatternType::Checkers => choose(point.x.floor() + point.y.floor() + point.z.floor()),
            PatternType::Test => colors::new(point.x, point.y, point.z)
        }
    }

    pub fn at_object<S>(&self, object: &Object<S>, world_point: Point) -> Color {
        let object_point = object.transformation.inverse_or_id44() * world_point;
        let pattern_point = self.transformation.inverse_or_id44() * object_point;
        self.at(pattern_point)
    }
}

impl CanTransform for Pattern {
    fn get_transformation(&self) -> &Matrix {
        &self.transformation
    }

    fn set_transformation(mut self, transformation: Matrix) -> Self {
        self.transformation = transformation;
        self
    }
}

#[cfg(test)]
mod tests;