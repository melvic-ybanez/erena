use crate::math::Real;
use crate::shapes::Shape;

struct Intersection<S: Shape> {
    t: Real,
    object: S,
}

impl<S: Shape> Intersection<S> {
    fn new(t: Real, object: S) -> Intersection<S> {
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