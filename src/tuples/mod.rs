use std::ops;

use crate::math::Real;
use crate::math;
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    x: Real,
    y: Real,
    z: Real,
    w: Real,
}

impl Tuple {
    fn new(x: Real, y: Real, z: Real, w: Real) -> Tuple {
        Tuple { x, y, z, w }
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

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        math::compare_reals(self.x, other.x) &&
            math::compare_reals(self.y, other.y) &&
            math::compare_reals(self.z, other.z) &&
            math::compare_reals(self.w, other.w)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector(Tuple);

impl Vector {
    fn new(x: Real, y: Real, z: Real) -> Vector {
        Vector(Tuple::new(x, y, z, 0.0))
    }

    fn zero() -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }

    fn normalize(&self) -> Vector {
        let magnitude = self.0.magnitude();
        Vector(Tuple::new(
            self.0.x / magnitude, self.0.y / magnitude, self.0.z / magnitude, self.0.w / magnitude)
        )
    }

    fn cross(&self, that: Vector) -> Vector {
        Vector::new(
            self.0.y * that.0.z - self.0.z * that.0.y,
            self.0.z * that.0.x - self.0.x * that.0.z,
            self.0.x * that.0.y - self.0.y * that.0.x,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Point(Tuple);

impl Point {
    fn new(x: Real, y: Real, z: Real) -> Point {
        Point(Tuple::new(x, y, z, 1.0))
    }
}

#[derive(Debug, PartialEq)]
pub struct Color(Tuple);

impl Color {
    fn new(red: Real, green: Real, blue: Real) -> Color {
        Color(Tuple::new(red, green, blue, 0.0))
    }

    fn red(&self) -> Real {
        self.0.x
    }

    fn green(&self) -> Real {
        self.0.y
    }

    fn blue(&self) -> Real {
        self.0.z
    }
}

impl ops::Mul for Color {
    type Output = Color;

    /// Hadamard product
    fn mul(self, other: Self) -> Self::Output {
        let r = self.red() * other.red();
        let g = self.green() * other.green();
        let b = self.blue() * other.blue();
        Color::new(r, g, b)
    }
}

#[cfg(test)]
mod tests;