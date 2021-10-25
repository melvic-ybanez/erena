use crate::tuples::colors::Color;
use crate::tuples::points::Point;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Stripe(Color, Color);

impl Stripe {
    pub fn new(first: Color, second: Color) -> Stripe {
        Stripe(first, second)
    }

    pub fn at(&self, point: Point) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
           self.0
        } else {
            self.1
        }
    }
}

#[cfg(test)]
mod tests;