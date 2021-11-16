pub mod random;

use std::cmp::Ordering;

/// Represents real number
pub type Real = f64;

pub const EPSILON: Real = 0.000001;

pub const PI: Real = std::f64::consts::PI;

pub type Idx = (usize, usize);

pub const DEG_45: Real = PI / 4.0;

pub(crate) fn compare_reals(a: Real, b: Real) -> bool {
    let diff = (a - b).abs();
    diff.is_nan() || diff < EPSILON
}

pub(crate) fn index_of(col: usize, row: usize, width: usize) -> usize {
    row * width + col
}

pub(crate) fn scale_to(max_value: i32, value: Real) -> i32 {
    let result = ((max_value as Real + 1.0) * value) as i32;
    if result > max_value {
        max_value
    } else if result < 0 {
        0
    } else {
        result
    }
}

pub(crate) fn round(value: Real, limit: u32) -> Real {
    let limit = 10f64.powf(limit as Real);
    (value * limit).round() / limit
}

pub(crate) fn round_to_5(value: Real) -> Real {
    round(value, 5)
}

pub fn order_reals(a: Real, b: Real) -> Ordering {
    if a < b {
        Ordering::Less
    } else if b < a {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

/// This is used a lot in tests
pub(crate) fn two_sqrt_div_2() -> Real {
    2_f64.sqrt() / 2.0
}
