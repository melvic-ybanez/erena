pub type Real = f64;

fn compare_reals(a: Real, b: Real) -> bool {
    (a - b).abs() <= Real::EPSILON
}