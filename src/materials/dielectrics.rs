use crate::rays::Comps3D;
use crate::math::Real;

pub fn schlick(comps: Comps3D) -> Real {
    // cosine of the angle between the eye and normal vectors
    let cos = comps.get_eye_vec().dot(comps.get_normal_vec());

    // total internal reflection occurs when n1 > n2
    let cos = if comps.get_n1() > comps.get_n2() {
        let n = comps.get_n1() / comps.get_n2();
        let sin2t = n * n * (1.0 - cos * cos);

        if sin2t > 1.0 {
            return 1.0
        }

        (1.0 - sin2t).sqrt()
    } else {
        cos
    };

    let r0 = ((comps.get_n1() - comps.get_n2()) / (comps.get_n1() + comps.get_n2())).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::spheres;
    use crate::rays::{Ray, Intersection, Comps};
    use crate::tuples::{points, vectors};
    use crate::math;
    use crate::tuples::points::Point;

    #[test]
    fn test_schlick_under_total_internal_reflection() {
        let shape = spheres::glass();
        let ray = Ray::new(points::new(0.0, 0.0, math::two_sqrt_div_2()), vectors::new(0.0, 1.0, 0.0));
        let xs = Intersection::from_data(&[
            (-math::two_sqrt_div_2(), &shape),
            (math::two_sqrt_div_2(), &shape)
        ]);
        let comps = Comps::prepare(&xs[1], &ray, &xs);
        let reflectance = schlick(comps);
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn test_schlick_with_perpendicular_angle() {
        let shape = spheres::glass();
        let ray = Ray::new(Point::origin(), vectors::new(0.0, 1.0, 0.0));
        let xs = Intersection::from_data(&[(-1.0, &shape), (1.0, &shape)]);
        let comps = Comps::prepare(&xs[1], &ray, &xs);
        let reflectance = schlick(comps);
        assert_eq!(math::round(reflectance, 2), 0.04);
    }

    /// Tests the schlick approximation with small angle and n2 > n1
    #[test]
    fn test_schlick_n2_over_n1() {
        let shape = spheres::glass();
        let ray = Ray::new(points::new(0.0, 0.99, -2.0), vectors::new(0.0, 0.0, 1.0));
        let xs = vec![Intersection::new(1.8589, &shape)];
        let comps = Comps::prepare(&xs[0], &ray, &xs);
        let reflectance = schlick(comps);
        assert_eq!(math::round(reflectance, 5), 0.48873);
    }
}