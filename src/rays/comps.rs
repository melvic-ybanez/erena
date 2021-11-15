use crate::math;
use crate::math::Real;
use crate::rays::{Intersection3D, Ray};
use crate::shapes::{Geo, Object, Shape};
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use std::borrow::Borrow;

#[derive(Clone)]
pub struct Comps<'a, S> {
    t: Real,
    object: &'a Object<S>,
    point: Point,
    eye_vec: Vector,
    normal_vec: Vector,
    reflect_vec: Vector,
    inside: bool,
    over_point: Option<Point>,
    under_point: Option<Point>,
    n1: Real,
    n2: Real,
}

pub type Comps3D<'a> = Comps<'a, Geo>;

impl<'a> Comps3D<'a> {
    pub fn prepare_default(hit: &'a Intersection3D, ray: &Ray) -> Comps3D<'a> {
        Comps3D::prepare(hit, ray, &vec![hit.clone()])
    }

    pub fn prepare(hit: &'a Intersection3D, ray: &Ray, xs: &[Intersection3D]) -> Comps3D<'a> {
        // same as the values of the corresponding intersection properties
        let t = hit.t;
        let object = hit.object.borrow();

        let point = ray.position(t);

        //let mut comps = Comps::new(t, object, point, -ray.direction, object.normal_at(point));
        let mut comps = Comps {
            t,
            object,
            point,
            eye_vec: -ray.direction,
            normal_vec: object.normal_at(point, hit),
            reflect_vec: Vector::zero(),
            inside: false,
            over_point: None,
            under_point: None,
            n1: 0.0,
            n2: 0.0,
        };

        if comps.normal_vec.dot(comps.eye_vec) < 0.0 {
            comps.inside = true;
            comps.normal_vec = -comps.normal_vec;
        } else {
            comps.inside = false;
        }

        comps.over_point = Some(comps.point + comps.normal_vec * math::EPSILON);
        comps.under_point = Some(comps.point - comps.normal_vec * math::EPSILON);
        comps.reflect_vec = ray.direction.reflect(comps.normal_vec);

        comps.compute_n1_and_n2(hit, xs);
        comps
    }

    fn compute_n1_and_n2(&mut self, hit: &Intersection3D, xs: &[Intersection3D]) {
        let mut containers: Vec<&Shape> = vec![];

        for i in xs.iter() {
            if i == hit {
                self.n1 = containers
                    .last()
                    .map(|obj| obj.material.refractive_index)
                    .unwrap_or(1.0);
            }

            if let Some(position) = containers.iter().position(|x| x == &i.object.borrow()) {
                containers.remove(position);
            } else {
                containers.push(&i.object);
            }

            if i == hit {
                self.n2 = containers
                    .last()
                    .map(|obj| obj.material.refractive_index)
                    .unwrap_or(1.0);
                break;
            }
        }
    }
}

impl<'a, S> Comps<'a, S> {
    pub fn get_over_point(&self) -> Point {
        Comps::<'a, S>::expect_or_invalid(self.over_point)
    }

    pub fn get_under_point(&self) -> Point {
        Comps::<'a, S>::expect_or_invalid::<Point>(self.under_point)
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

    pub fn get_n1(&self) -> Real {
        self.n1
    }

    pub fn get_n2(&self) -> Real {
        self.n2
    }

    fn expect_or_invalid<A>(opt: Option<A>) -> A {
        opt.expect("Invalid state")
    }
}

#[cfg(test)]
mod tests {
    use crate::rays::comps::{Comps, Comps3D};
    use crate::rays::{Intersection, Ray};
    use crate::shapes::Shape;
    use crate::tuples::points::Point;
    use crate::tuples::{points, vectors};
    use std::borrow::Borrow;
    use std::rc::Rc;

    /// Tests precomputing the state of an intersection
    #[test]
    fn test_intersection_state() {
        let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
        let shape = Rc::new(Shape::sphere());
        let i = Intersection::from_ref(4.0, &shape);
        let comps = Comps::prepare_default(&i, &ray);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object.borrow());
        assert_eq!(comps.point, points::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_vec, vectors::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_vec, vectors::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_outside_intersection() {
        let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
        let shape = Rc::new(Shape::sphere());
        let i = Intersection::from_ref(4.0, &shape);
        let comps = Comps3D::prepare_default(&i, &ray);
        assert!(!comps.inside);
    }

    #[test]
    fn test_inside_intersection() {
        let ray = Ray::new(Point::origin(), vectors::new(0.0, 0.0, 1.0));
        let shape = Rc::new(Shape::sphere());
        let i = Intersection::from_ref(1.0, &shape);
        let comps = Comps3D::prepare_default(&i, &ray);

        assert_eq!(comps.point, points::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_vec, vectors::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normal_vec, vectors::new(0.0, 0.0, -1.0)); // (0, 0, 1) but inverted
    }
}
