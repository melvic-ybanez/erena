use std::ops;

use crate::math::Real;
use crate::{math, tuples};
use std::ops::Index;
use std::marker::PhantomData;
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
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn magnitude(&self) -> Real {
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
        (0..4).all(|i| math::compare_reals(self[i], other[i]))
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct TupleT;
pub type Tuple = TupleLike<TupleT>;

pub fn new(x: Real, y: Real, z: Real, w: Real) -> Tuple {
    TupleLike::new(x, y, z, w)
}

pub(crate) mod vectors {
    use crate::tuples::TupleLike;
    use crate::math::Real;

    #[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
    pub struct VectorT;
    pub type Vector = TupleLike<VectorT>;

    pub fn new(x: Real, y: Real, z: Real) -> Vector {
        TupleLike::new(x, y, z, 0.0)
    }

    impl Vector {
        #[inline(always)]
        pub(crate) fn zero() -> Vector {
            new(0.0, 0.0, 0.0)
        }

        pub(crate) fn normalize(&self) -> Vector {
            let magnitude = self.magnitude();
            TupleLike::new(
                self.x / magnitude, self.y / magnitude, self.z / magnitude, self.w / magnitude
            )
        }

        pub(crate) fn cross(&self, that: Vector) -> Vector {
            new(
                self.y * that.z - self.z * that.y,
                self.z * that.x - self.x * that.z,
                self.x * that.y - self.y * that.x,
            )
        }
    }
}

pub(crate) mod points {
    use crate::tuples::TupleLike;
    use crate::math::Real;

    #[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
    pub struct PointT;
    pub type Point = TupleLike<PointT>;

    pub fn new(x: Real, y: Real, z: Real) -> Point {
        TupleLike::new(x, y, z, 1.0)
    }

    impl Point {
        #[inline(always)]
        pub(crate) fn origin() -> Point {
            new(0.0, 0.0, 0.0)
        }
    }
}

pub(crate) mod colors {
    use crate::tuples::TupleLike;
    use crate::math::Real;
    use std::ops;

    #[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
    pub struct ColorT;
    pub type Color = TupleLike<ColorT>;

    pub fn new(red: Real, green: Real, blue: Real) -> Color {
        TupleLike::new(red, green, blue, 0.0)
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