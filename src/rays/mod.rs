use crate::tuples::{Point, Vector};

struct Ray {
    origin: Point,
    direction: Vector
}

impl Ray {
    fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::{Point, Vector};
    use crate::rays::Ray;

    #[test]
    fn test_creating_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }
}