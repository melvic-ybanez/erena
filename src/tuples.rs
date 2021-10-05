use crate::math::Real;

#[derive(Debug, PartialEq)]
pub struct Tuple {
    x: Real,
    y: Real,
    z: Real,
    w: Real,
}

impl Tuple {
    fn new(x: Real, y: Real, z: Real, w: Real) -> Tuple {
        Tuple { x, y, z, w }
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

mod point {
    use crate::tuples::Tuple;
    use crate::math::Real;

    pub struct Point(Tuple);

    impl Point {
        pub fn new(x: Real, y: Real, z: Real) -> Tuple {
            Tuple::new(x, y, z, 1.0)
        }
    }
}

mod vector {
    use crate::math::Real;
    use crate::tuples::Tuple;

    pub struct Vector(Tuple);

    impl Vector {
        pub fn new(x: Real, y: Real, z: Real) -> Tuple {
            Tuple::new(x, y, z, 0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::Tuple;
    use crate::tuples::point::Point;
    use crate::tuples::vector::Vector;

    /// A tuple with w=1.0 is a point
    #[test]
    fn test_point_w() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 1.0);
        assert!(tuple.is_point());
    }

    /// A tuple with w=0 is a vector
    #[test]
    fn test_vector_w() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 0.0);
        assert!(tuple.is_vector());
    }

    #[test]
    fn test_point_creation() {
        let point = Point::new(4.0, -4.0, 3.0);
        assert_eq!(point, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn test_vector_creation() {
        let vector = Vector::new(4.0, -4.0, 3.0);
        assert_eq!(vector, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }
}