use crate::math::Real;
use crate::shapes::Shape;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Intersection<S: Shape> {
    pub(crate) t: Real,
    pub(crate) object: S,
}

impl<S: Shape> Intersection<S> {
    pub(crate) fn new(t: Real, object: S) -> Intersection<S> {
        Intersection { t, object }
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
}