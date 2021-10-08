use std::ops;

use crate::math::Real;
use crate::math;
use std::ops::Index;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: Real,
    pub y: Real,
    pub z: Real,
    pub w: Real,
}

impl Tuple {
    pub(crate) const LEN: usize = 4;

    pub(crate) fn new(x: Real, y: Real, z: Real, w: Real) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub(crate) fn from_array(elems: &[Real; Tuple::LEN]) -> Tuple {
        Tuple::new(elems[0], elems[1], elems[2], elems[3])
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

impl Index<usize> for Tuple {
    type Output = Real;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Invalid index")
        }
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

    #[inline(always)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color(pub Tuple);

impl Color {
    pub(crate) fn new(red: Real, green: Real, blue: Real) -> Color {
        Color(Tuple::new(red, green, blue, 0.0))
    }

    pub(crate) fn red_value(&self) -> Real {
        self.0.x
    }

    pub(crate) fn green_value(&self) -> Real {
        self.0.y
    }

    pub(crate) fn blue_value(&self) -> Real {
        self.0.z
    }

    #[inline(always)]
    pub(crate) fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    #[inline(always)]
    pub(crate) fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }
}

impl ops::Mul for Color {
    type Output = Color;

    /// Hadamard product
    fn mul(self, other: Self) -> Self::Output {
        let r = self.red_value() * other.red_value();
        let g = self.green_value() * other.green_value();
        let b = self.blue_value() * other.blue_value();
        Color::new(r, g, b)
    }
}

#[cfg(test)]
mod tests;