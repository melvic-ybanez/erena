use std::marker::PhantomData;
use std::ops;
use std::ops::Index;

use crate::{math, tuples};
use crate::math::Real;
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;

#[derive(Debug, Copy, Clone, PartialOrd)]
pub struct TupleLike<T> {
    pub x: Real,
    pub y: Real,
    pub z: Real,
    pub w: Real,
    _phantom: PhantomData<T>
}

pub const LEN: usize = 4;

impl<T> TupleLike<T> {

    pub(crate) fn new(x: Real, y: Real, z: Real, w: Real) -> TupleLike<T> {
        TupleLike { x, y, z, w, _phantom: PhantomData }
    }

    pub(crate) fn from_array(elems: &[Real; LEN]) -> TupleLike<T> {
        TupleLike::new(elems[0], elems[1], elems[2], elems[3])
    }

    fn is_point(&self) -> bool {
        self.w == Point::W
    }

    fn is_vector(&self) -> bool {
        self.w == Vector::W
    }

    pub fn magnitude(&self) -> Real {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn dot<S>(&self, that: TupleLike<S>) -> Real {
        self.x * that.x + self.y * that.y + self.z * that.z + self.w * that.w
    }

    pub fn to_vector(&self) -> Vector {
        vectors::new(self.x, self.y, self.z)
    }

    pub fn to_tuple(&self) -> Tuple {
        tuples::new(self.x, self.y, self.z, self.w)
    }

    pub fn to_point(&self) -> Point {
        points::new(self.x, self.y, self.z)
    }

    /// This is primarily used for testing
    pub(crate) fn round_items(&self) -> TupleLike<T> {
        TupleLike::new(
            math::round_to_5(self.x),
            math::round_to_5(self.y),
            math::round_to_5(self.z),
            math::round_to_5(self.w)
        )
    }
}

impl<T, S> ops::Add<TupleLike<S>> for TupleLike<T> {
    type Output = TupleLike<T>;

    fn add(self, that: TupleLike<S>) -> Self::Output {
        TupleLike::new(self.x + that.x, self.y + that.y, self.z + that.z, self.w + that.w)
    }
}

impl<T, S> ops::Sub<TupleLike<S>> for TupleLike<T> {
    type Output = TupleLike<T>;

    fn sub(self, rhs: TupleLike<S>) -> Self::Output {
        TupleLike::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl<T> ops::Neg for TupleLike<T> {
    type Output = TupleLike<T>;

    fn neg(self) -> Self::Output {
        TupleLike::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl<T> ops::Mul<Real> for TupleLike<T> {
    type Output = TupleLike<T>;

    /// Scalar multiplication
    fn mul(self, scalar: Real) -> Self::Output {
        TupleLike::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
}

impl<T> ops::Div<Real> for TupleLike<T> {
    type Output = TupleLike<T>;

    fn div(self, scalar: Real) -> Self::Output {
        self * (1.0 / scalar)
    }
}

impl<T> Index<usize> for TupleLike<T> {
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

impl<T> PartialEq for TupleLike<T> {
    fn eq(&self, other: &Self) -> bool {
        (0..LEN).all(|i| math::compare_reals(self[i], other[i]))
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct TupleT;
pub type Tuple = TupleLike<TupleT>;

pub fn new(x: Real, y: Real, z: Real, w: Real) -> Tuple {
    TupleLike::new(x, y, z, w)
}

pub(crate) mod vectors {
    use crate::math::Real;
    use crate::tuples::TupleLike;

    #[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
    pub struct VectorT;
    pub type Vector = TupleLike<VectorT>;

    pub fn new(x: Real, y: Real, z: Real) -> Vector {
        TupleLike::new(x, y, z, Vector::W)
    }

    impl Vector {
        pub const W: Real = 0.0;

        #[inline(always)]
        pub(crate) fn zero() -> Vector {
            new(0.0, 0.0, 0.0)
        }

        pub fn normalize(&self) -> Vector {
            let magnitude = self.magnitude();
            if magnitude == 0.0 {
                *self
            } else {
                new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
            }
        }

        pub fn cross(&self, that: Vector) -> Vector {
            new(
                self.y * that.z - self.z * that.y,
                self.z * that.x - self.x * that.z,
                self.x * that.y - self.y * that.x,
            )
        }

        pub fn reflect(&self, normal: Vector) -> Vector {
            *self - normal * 2.0 * self.dot(normal)
        }
    }
}

pub(crate) mod points {
    use crate::math::Real;
    use crate::tuples::TupleLike;
    use crate::tuples::vectors::Vector;

    #[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
    pub struct PointT;
    pub type Point = TupleLike<PointT>;

    pub fn new(x: Real, y: Real, z: Real) -> Point {
        TupleLike::new(x, y, z, Point::W)
    }

    impl Point {
        pub const W: Real = 1.0;

        #[inline(always)]
        pub(crate) fn origin() -> Point {
            new(0.0, 0.0, 0.0)
        }

        pub fn normalize(&self) -> Vector {
            self.to_vector().normalize()
        }
    }
}

pub(crate) mod colors {
    use std::ops;

    use crate::math::Real;
    use crate::tuples::TupleLike;

    #[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
    pub struct ColorT;
    pub type Color = TupleLike<ColorT>;

    pub fn new(red: Real, green: Real, blue: Real) -> Color {
        TupleLike::new(red, green, blue, 0.0)
    }

    pub fn rgbi(r: i32, g: i32, b: i32) -> Color {
        new(r as Real / 255.0, g as Real / 255.0, b as Real / 255.0)
    }

    impl Color {
        pub(crate) fn red_value(&self) -> Real {
            self.x
        }

        pub(crate) fn green_value(&self) -> Real {
            self.y
        }

        pub(crate) fn blue_value(&self) -> Real {
            self.z
        }

        #[inline(always)]
        pub(crate) fn black() -> Color {
            new(0.0, 0.0, 0.0)
        }

        #[inline(always)]
        pub(crate) fn red() -> Color {
            new(1.0, 0.0, 0.0)
        }

        #[inline(always)]
        pub fn white() -> Color {
            new(1.0, 1.0, 1.0)
        }
    }

    impl ops::Mul for Color {
        type Output = Color;

        /// Hadamard product
        fn mul(self, other: Self) -> Self::Output {
            let r = self.red_value() * other.red_value();
            let g = self.green_value() * other.green_value();
            let b = self.blue_value() * other.blue_value();
            new(r, g, b)
        }
    }
}

#[cfg(test)]
mod tests;