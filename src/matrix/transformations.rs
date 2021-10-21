use crate::math::Real;
use crate::matrix::Matrix;
use crate::tuples::vectors::Vector;
use crate::tuples::points::Point;

#[inline(always)]
pub fn translation(x: Real, y: Real, z: Real) -> Matrix {
    Matrix::new44(&[
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0,
    ])
}

#[inline(always)]
pub fn scaling(x: Real, y: Real, z: Real) -> Matrix {
    Matrix::new44(&[
        x, 0.0, 0.0, 0.0,
        0.0, y, 0.0, 0.0,
        0.0, 0.0, z, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

#[inline(always)]
pub fn rotation_x(rad: Real) -> Matrix {
    Matrix::new44(&[
        1.0, 0.0, 0.0, 0.0,
        0.0, rad.cos(), -rad.sin(), 0.0,
        0.0, rad.sin(), rad.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

#[inline(always)]
pub fn rotation_y(rad: Real) -> Matrix {
    Matrix::new44(&[
        rad.cos(), 0.0, rad.sin(), 0.0,
        0.0, 1.0, 0.0, 0.0,
        -rad.sin(), 0.0, rad.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

#[inline(always)]
pub fn rotation_z(rad: Real) -> Matrix {
    Matrix::new44(&[
        rad.cos(), -rad.sin(), 0.0, 0.0,
        rad.sin(), rad.cos(), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

#[inline(always)]
pub fn shearing(
    xy: Real, xz: Real, yx: Real,
    yz: Real, zx: Real, zy: Real,
) -> Matrix {
    Matrix::new44(&[
        1.0, xy, xz, 0.0,
        yx, 1.0, yz, 0.0,
        zx, zy, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

pub fn view_transformation(from: Point, to: Point, up: Vector) -> Matrix {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let true_up = left.cross(forward);
    let orientation = Matrix::new44(&[
        left.x, left.y, left.z, 0.0,
        true_up.x, true_up.y, true_up.z, 0.0,
        -forward.x, -forward.y, -forward.z, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]);
    orientation * translation(-from.x, -from.y, -from.z)
}