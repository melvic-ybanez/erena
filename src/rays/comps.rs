use crate::math::Real;
use crate::shapes::{Object, Space3D};
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::rays::{Intersection, Ray};

pub struct Comps<'a, S> {
    pub t: Real,
    pub object: &'a Object<S>,
    pub point: Point,
    pub eye_vec: Vector,
    pub normal_vec: Vector,
    pub inside: bool,
}

impl<'a, S> Comps<'a, S> {
    pub fn new(t: Real, object: &Object<S>, point: Point, eye_vec: Vector, normal_vec: Vector) -> Comps<S> {
        Comps { t, object, point, eye_vec, normal_vec, inside: false }
    }
}

pub type Comps3D<'a> = Comps<'a, Space3D>;

impl<'a> Comps3D<'a> {
    fn prepare(intersection: Intersection<'a, Space3D>, ray: &Ray) -> Comps3D<'a> {
        // same as the values of the corresponding intersection properties
        let t = intersection.t;
        let object = intersection.object;

        let point = ray.position(t);

        let mut comps = Comps::new(t, object, point, -ray.direction, object.normal_at(point));

        if comps.normal_vec.dot(comps.eye_vec) < 0.0 {
            comps.inside = true;
            comps.normal_vec = -comps.normal_vec;
        } else {
            comps.inside = false;
        }

        comps
    }
}

#[cfg(test)]
mod tests {
    use crate::rays::{Ray, Intersection};
    use crate::tuples::{points, vectors};
    use crate::shapes::Shape;
    use crate::rays::comps::{Comps, Comps3D};
    use crate::tuples::points::Point;

    /// Tests precomputing the state of an intersection
    #[test]
    fn test_intersection_state() {
        let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
        let shape = Shape::sphere();
        let i = Intersection::new(4.0, &shape);
        let comps = Comps::prepare(i, &ray);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, points::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_vec, vectors::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_vec, vectors::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_outside_intersection() {
        let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
        let shape = Shape::sphere();
        let i = Intersection::new(4.0, &shape);
        let comps = Comps3D::prepare(i, &ray);
        assert!(!comps.inside);
    }

    #[test]
    fn test_inside_intersection() {
        let ray = Ray::new(Point::origin(), vectors::new(0.0, 0.0, 1.0));
        let shape = Shape::sphere();
        let i = Intersection::new(1.0, &shape);
        let comps = Comps3D::prepare(i, &ray);

        assert_eq!(comps.point, points::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_vec, vectors::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normal_vec, vectors::new(0.0, 0.0, -1.0));     // (0, 0, 1) but inverted
    }
}