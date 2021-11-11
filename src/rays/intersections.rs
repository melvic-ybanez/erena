use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

use crate::math;
use crate::math::Real;
use crate::shapes::{Object, Geo, Shape};
use std::rc::Rc;


#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<S> {
    pub t: Real,
    pub object: Rc<Object<S>>,
    kind: IntersectionKind,
}

pub type Intersection3D = Intersection<Geo>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IntersectionKind {
    Regular,
    Triangle {
        u: Real,
        v: Real,
    },
}

impl<S: Clone + PartialEq> Intersection<S> {
    pub fn new(t: Real, object: Rc<Object<S>>) -> Intersection<S> {
        Intersection::new_with_kind(t, object, IntersectionKind::Regular)
    }

    pub fn new_with_kind(t: Real, object: Rc<Object<S>>, kind: IntersectionKind) -> Intersection<S> {
        Intersection { t, object, kind }
    }

    pub fn from_ref(t: Real, object: &Rc<Object<S>>) -> Intersection<S> {
        Intersection::new(t, Rc::clone(object))
    }

    pub fn hit_from_refs(xs: Vec<&Intersection<S>>) -> Option<Intersection<S>> {
        let mut xs: Vec<_> = xs.into_iter().filter(|x| x.t >= 0.0).collect();
        if xs.is_empty() {
            None
        } else {
            xs.sort_by(|x, y| x.t.partial_cmp(&y.t).unwrap_or(Equal));
            Some(xs[0].clone())
        }
    }

    pub fn hit(xs: Vec<Intersection<S>>) -> Option<Intersection<S>> {
        Intersection::hit_from_refs(xs.iter().collect::<Vec<_>>())
    }

    pub fn agg(shape: &Rc<Object<S>>, ts: &[Real]) -> Vec<Intersection<S>> {
        ts.iter().map(|&t| Intersection::from_ref(t, shape)).collect()
    }

    pub fn from_data(data: &[(Real, &Rc<Object<S>>)]) -> Vec<Intersection<S>> {
        data.iter().map(|(t, obj)| Intersection::from_ref(*t, obj)).collect()
    }

    pub(crate) fn compare(i1: &Intersection<S>, i2: &Intersection<S>) -> Ordering {
        math::order_reals(i1.t, i2.t)
    }

    pub fn get_kind(&self) -> IntersectionKind {
        self.kind
    }
}

impl Intersection3D {
    pub fn new_with_uv(t: Real, shape: Rc<Shape>, u: Real, v: Real) -> Intersection3D {
        Intersection::new_with_kind(t, Rc::clone(&shape), if let Geo::Triangle(_) = shape.geo {
            IntersectionKind::Triangle { u, v }
        } else {
            IntersectionKind::Regular
        })
    }

    pub fn test() -> Intersection3D {
        Intersection::new(0.0, Rc::new(Shape::test()))
    }
}

#[cfg(test)]
mod tests {
    use crate::rays::intersections::{Intersection, IntersectionKind};
    use crate::shapes::{Shape, spheres};
    use crate::rays::{Ray, Comps};
    use crate::tuples::{points, vectors};
    use crate::matrix::CanTransform;
    use crate::math;
    use crate::materials::Material;
    use std::rc::Rc;

    #[test]
    fn test_intersection_fields() {
        let sphere = Rc::new(Shape::sphere());
        let i = Intersection::from_ref(3.5, &sphere);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, sphere);
    }

    /// Tests the hit when all intersections have positive t
    #[test]
    fn test_hit_when_all_ts_are_positive() {
        let sphere = Rc::new(Shape::sphere());
        let i1 = Intersection::from_ref(1.0, &sphere);
        let i2 = Intersection::from_ref(2.0, &sphere);
        let xs = vec![&i1, &i2];
        assert_eq!(Intersection::hit_from_refs(xs), Some(i1));
    }

    #[test]
    fn test_hit_when_some_ts_are_negative() {
        let sphere = Rc::new(Shape::sphere());
        let xs = Intersection::agg(&sphere, &[-1.0, 1.0]);
        assert_eq!(Intersection::hit(xs), Some(Intersection::from_ref(1.0, &sphere)));
    }

    #[test]
    fn test_hit_when_all_t_are_negative() {
        let sphere = Rc::new(Shape::sphere());
        let i1 = Intersection::from_ref(-2.0, &sphere);
        let i2 = Intersection::from_ref(-1.0, &sphere);
        let xs = vec![&i2, &i1];
        assert_eq!(Intersection::hit_from_refs(xs), None);
    }

    /// Tests the hit as being the lowest non-negative intersection
    #[test]
    fn test_hit_as_lowest_non_negative() {
        let sphere = Rc::new(Shape::sphere());
        let xs = Intersection::agg(&sphere, &[
            5.0, 7.0, -3.0, 2.0,
        ]);
        assert_eq!(Intersection::hit(xs), Some(Intersection::from_ref(2.0, &sphere)));
    }

    /// Tests that the hit should offset the point
    #[test]
    fn test_hit_offset() {
        let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
        let shape = Rc::new(Shape::sphere().translate(0.0, 0.0, 1.0));
        let i = Intersection::from_ref(5.0, &shape);
        let comps = Comps::prepare_default(&i, &ray);

        assert!(comps.get_over_point().z < -math::EPSILON / 2.0);
        assert!(comps.get_point().z > comps.get_over_point().z);
    }

    #[test]
    fn test_precompute_reflection_vector() {
        let shape = Rc::new(Shape::plane());
        let ray = Ray::new(
            points::new(0.0, 1.0, 1.0),
            vectors::new(0.0, -math::two_sqrt_div_2(), math::two_sqrt_div_2()),
        );
        let i = Intersection::from_ref(2_f64.sqrt(), &shape);
        let comps = Comps::prepare_default(&i, &ray);
        assert_eq!(comps.get_reflect_vec(), vectors::new(0.0, math::two_sqrt_div_2(), math::two_sqrt_div_2()));
    }

    #[test]
    fn test_finding_n1_and_n2() {
        let a = Rc::new(spheres::glass()
            .scale(2.0, 2.0, 2.0)
            .material(Material::default().refractive_index(1.5)));
        let b = Rc::new(spheres::glass()
            .translate(0.0, 0.0, -0.25)
            .material(Material::default().refractive_index(2.0)));
        let c = Rc::new(spheres::glass()
            .translate(0.0, 0.0, 0.25)
            .material(Material::default().refractive_index(2.5)));
        let ray = Ray::new(points::new(0.0, 0.0, -4.0), vectors::new(0.0, 0.0, 1.0));
        let xs = Intersection::from_data(&[
            (2.0, &a),
            (2.75, &b),
            (3.25, &c),
            (4.75, &b),
            (5.25, &c),
            (6.0, &a),
        ]);
        let samples = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];
        for i in 0..samples.len() {
            let comps = Comps::prepare(&xs[i], &ray, &xs.clone());
            assert_eq!(comps.get_n1(), samples[i].0);
            assert_eq!(comps.get_n2(), samples[i].1);
        }
    }

    /// The under point is offset below the surface
    #[test]
    fn test_under_point_offset_below() {
        let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
        let shape = Rc::new(spheres::glass().translate(0.0, 0.0, 1.0));
        let i = Intersection::from_ref(5.0, &shape);
        let comps = Comps::prepare(&i, &ray, &vec![i.clone()]);
        assert!(comps.get_under_point().z > math::EPSILON / 2.0);
        assert!(comps.get_point().z < comps.get_under_point().z);
    }

    #[test]
    fn test_with_uv() {
        let shape = Shape::triangle(
            points::new(0.0, 1.0, 0.0),
            points::new(-1.0, 0.0, 0.0),
            points::new(1.0, 0.0, 0.0),
        );
        let intersection = Intersection::new_with_uv(3.5, Rc::new(shape), 0.2, 0.4);

        if let IntersectionKind::Triangle { u, v } = intersection.get_kind() {
            assert_eq!(u, 0.2);
            assert_eq!(v, 0.4);
        } else {
            panic!("Intersection kind is not triangle");
        }
    }
}