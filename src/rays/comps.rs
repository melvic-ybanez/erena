use crate::math::Real;
use crate::rays::{Intersection, Ray, Intersection3D};
use crate::shapes::{Object, Space3D, Shape};
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::math;

pub struct Comps<'a, S> {
    t: Real,
    object: &'a Object<S>,
    point: Point,
    eye_vec: Vector,
    normal_vec: Vector,
    reflect_vec: Vector,
    inside: bool,
    over_point: Option<Point>,
}

pub type Comps3D<'a> = Comps<'a, Space3D>;

impl<'a> Comps3D<'a> {
    pub fn prepare(intersection: Intersection3D<'a>, ray: &Ray) -> Comps3D<'a> {
        // same as the values of the corresponding intersection properties
        let t = intersection.t;
        let object = intersection.object;

        let point = ray.position(t);

        //let mut comps = Comps::new(t, object, point, -ray.direction, object.normal_at(point));
        let mut comps = Comps {
            t,
            object,
            point,
            eye_vec: -ray.direction,
            normal_vec: object.normal_at(point),
            reflect_vec: Vector::zero(),
            inside: false,
            over_point: None,
        };

        if comps.normal_vec.dot(comps.eye_vec) < 0.0 {
            comps.inside = true;
            comps.normal_vec = -comps.normal_vec;
        } else {
            comps.inside = false;
        }

        comps.over_point = Some(comps.point + comps.normal_vec * math::EPSILON);
        comps.reflect_vec = ray.direction.reflect(comps.normal_vec);

        comps
    }
}
impl<'a, S> Comps<'a, S> {
    pub fn get_overpoint(&self) -> Point {
        self.over_point.expect("Invalid state")
    }

    pub fn get_reflect_vec(&self) -> Vector {
        self.reflect_vec
    }

    pub fn get_eye_vec(&self) -> Vector {
        self.eye_vec
    }

    pub fn get_object(&self) -> &'a Object<S> {
        self.object
    }

    pub fn get_point(&self) -> Point {
        self.point
    }

    pub fn get_normal_vec(&self) -> Vector {
        self.normal_vec
    }
}

#[cfg(test)]
mod tests {
    use crate::rays::{Intersection, Ray};
    use crate::rays::comps::{Comps, Comps3D};
    use crate::shapes::Shape;
    use crate::tuples::{points, vectors};
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