use crate::tuples::colors::Color;
use crate::tuples::points::Point;
use crate::matrix::{CanTransform, Matrix};
use crate::shapes::Object;

#[derive(Clone, PartialEq, Debug)]
pub struct Stripe(Color, Color, Matrix);

impl Stripe {
    pub fn new(first: Color, second: Color) -> Stripe {
        Stripe(first, second, Matrix::id44())
    }

    pub fn at(&self, point: Point) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
           self.0
        } else {
            self.1
        }
    }

    pub fn at_object<S>(&self, object: &Object<S>, world_point: Point) -> Color {
        let object_point = object.transformation.inverse_or_id44() * world_point;
        let pattern_point = self.2.inverse_or_id44() * object_point;
        self.at(pattern_point)
    }
}

impl CanTransform for Stripe {
    fn get_transformation(&self) -> &Matrix {
        &self.2
    }

    fn set_transformation(mut self, transformation: Matrix) -> Self {
        self.2 = transformation;
        self
    }
}

#[cfg(test)]
mod tests;