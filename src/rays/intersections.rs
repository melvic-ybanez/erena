use crate::math::Real;
use crate::shapes::Shape;
use std::cmp::Ordering::Equal;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Intersection<S: Shape> {
    pub(crate) t: Real,
    pub(crate) object: S,
}

impl<S: Shape + Copy> Intersection<S> {
    pub(crate) fn new(t: Real, object: S) -> Intersection<S> {
        Intersection { t, object }
    }

    pub(crate) fn hit(xs: Vec<Intersection<S>>) -> Option<Intersection<S>> {
        let mut xs: Vec<_> = xs.into_iter().filter(|x| x.t >= 0.0).collect();
        if xs.is_empty() {
            None
        } else {
            xs.sort_by(|x, y| x.t.partial_cmp(&y.t).unwrap_or(Equal));
            Some(xs[0])
        }
    }

    pub(crate) fn agg(shape: S, ts: &[Real]) -> Vec<Intersection<S>> {
        ts.iter().map(|&t| Intersection::new(t, shape)).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::Sphere;
    use crate::rays::intersections::Intersection;

    #[test]
    fn test_intersection_fields() {
        let sphere = Sphere::new();
        let i = Intersection::new(3.5, sphere);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, sphere);
    }

    /// Tests the hit when all intersections have positive t
    #[test]
    fn test_hit_when_all_ts_are_positive() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(1.0, sphere);
        let i2 = Intersection::new(2.0, sphere);
        let xs = vec![i1, i2];
        assert_eq!(Intersection::hit(xs), Some(i1));
    }

    #[test]
    fn test_hit_when_some_ts_are_negative() {
        let sphere = Sphere::new();
        let xs = Intersection::agg(sphere, &[-1.0, 1.0]);
        assert_eq!(Intersection::hit(xs), Some(Intersection::new(1.0, sphere)));
    }

    #[test]
    fn test_hit_when_all_t_are_negative() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(-2.0, sphere);
        let i2 = Intersection::new(-1.0, sphere);
        let xs = vec![i2, i1];
        assert_eq!(Intersection::hit(xs), None);
    }

    /// Tests the hit as being the lowest non-negative intersection
    #[test]
    fn test_hit_as_lowest_non_negative() {
        let sphere = Sphere::new();
        let xs = Intersection::agg(sphere, &[
            5.0, 7.0, -3.0, 2.0,
        ]);
        assert_eq!(Intersection::hit(xs), Some(Intersection::new(2.0, sphere)));
    }
}