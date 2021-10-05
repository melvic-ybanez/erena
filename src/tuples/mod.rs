use crate::math::Real;
use std::ops;
use std::ops::Mul;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tuple {
    x: Real,
    y: Real,
    z: Real,
    w: Real,
}

type Vector = Tuple;
type Point = Tuple;
type Color = Tuple;

impl Tuple {
    fn new(x: Real, y: Real, z: Real, w: Real) -> Tuple {
        Tuple { x, y, z, w }
    }

    fn vector(x: Real, y: Real, z: Real) -> Vector {
        Tuple::new(x, y, z, 0.0)
    }

    fn point(x: Real, y: Real, z: Real) -> Point {
        Tuple::new(x, y, z, 1.0)
    }

    fn color(x: Real, y: Real, z: Real) -> Color {
        Tuple::new(x, y, z, 0.0)
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn magnitude(&self) -> Real {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    fn dot(&self, that: Tuple) -> Real {
        self.x * that.x + self.y * that.y + self.z * that.z + self.w * that.w
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, that: Tuple) -> Self::Output {
        Tuple::new(self.x + that.x, self.y + that.y, self.z + that.z, self.w + that.w)
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, that: Tuple) -> Self::Output {
        Tuple::new(self.x - that.x, self.y - that.y, self.z - that.z, self.w - that.w)
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl ops::Mul<Real> for Tuple {
    type Output = Tuple;

    /// Scalar multiplication
    fn mul(self, scalar: Real) -> Self::Output {
        Tuple::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
}

impl ops::Div<Real> for Tuple {
    type Output = Tuple;

    fn div(self, scalar: Real) -> Self::Output {
        self * (1.0 / scalar)
    }
}

impl Vector {
    fn zero() -> Vector {
        Tuple::vector(0.0, 0.0, 0.0)
    }

    fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        Tuple::new(self.x / magnitude, self.y / magnitude, self.z / magnitude, self.w / magnitude)
    }

    fn cross(&self, that: Vector) -> Vector {
        vector(
            self.y * that.z - self.z * that.y,
            self.z * that.x - self.x * that.z,
            self.x * that.y - self.y * that.x,
        )
    }
}

impl Color {
    fn red(color: Color) -> Real {
        color.x
    }

    fn green(color: Color) -> Real {
        color.y
    }

    fn blue(color: Color) -> Real {
        color.z
    }
}

fn vector(x: Real, y: Real, z: Real) -> Vector {
    Tuple::vector(x, y, z)
}

fn point(x: Real, y: Real, z: Real) -> Point {
    Tuple::point(x, y, z)
}

fn color(x: Real, y: Real, z: Real) -> Color {
    Tuple::color(x, y, z)
}

#[cfg(test)]
mod tests;