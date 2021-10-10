pub type Real = f64;

const EPSILON: Real = 0.000001;

pub(crate) const PI: Real = std::f64::consts::PI;

pub(crate) type Idx = (usize, usize);

pub fn compare_reals(a: Real, b: Real) -> bool {
    (a - b).abs() <= EPSILON
}

pub fn index_of(col: usize, row: usize, width: usize) -> usize {
    row * width + col
}

pub fn scale_to(max_value: i32, value: Real) -> i32 {
    let result = ((max_value as Real + 1.0) * value) as i32;
    if result > max_value {
        max_value
    } else if result < 0 {
        0
    } else {
        result
    }
}

pub fn round(value: Real, limit: u32) -> Real {
    let limit = 10f64.powf(limit as Real);
    (value * limit).round() / limit
}