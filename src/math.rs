pub type Real = f64;

pub fn compare_reals(a: Real, b: Real) -> bool {
    (a - b).abs() <= Real::EPSILON
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