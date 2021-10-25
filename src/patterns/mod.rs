use crate::tuples::colors::Color;
use crate::tuples::points::Point;
use crate::matrix::{CanTransform, Matrix};
use crate::shapes::Object;
use crate::patterns::PatternType::Stripe;

#[derive(Clone, PartialEq, Debug)]
pub struct Pattern {
    pub pattern_type: PatternType,
    pub transformation: Matrix,
}

#[derive(Clone, PartialEq, Debug)]
pub enum PatternType {
    Stripe(Color, Color),
}

impl Pattern {
    fn new(pattern_type: PatternType) -> Pattern {
        Pattern { pattern_type, transformation: Matrix::id44() }
    }

    pub fn stripe(first: Color, second: Color) -> Pattern {
        Pattern::new(Stripe(first, second))
    }

    pub fn at(&self, point: Point) -> Color {
        match self.pattern_type {
            Stripe(first, second) =>
                if point.x.floor() % 2.0 == 0.0 {
                    first
                } else {
                    second
                }
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