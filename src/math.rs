pub type Real = f64;

pub(crate) fn compare_reals(a: Real, b: Real) -> bool {
    (a - b).abs() <= Real::EPSILON
}