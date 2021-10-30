use crate::shapes::{Object, Geometry};
use crate::shapes::Geometry::Cylinder;
use crate::shapes::cylinders::CylLike;

#[derive(Debug, PartialEq, Clone)]
pub enum Group<G> {
    Leaf(G),
    Tree(Vec<Object<G>>)
}

impl Group<Geometry> {
    pub fn is_cone(&self) -> bool {
        if let Group::Leaf(Cylinder(cyl @ CylLike { .. })) = self {
            return cyl.is_cone()
        }
        return false
    }

    pub fn is_empty(&self) -> bool {
        if let Group::Tree(objects) = self {
            return objects.is_empty()
        }
        panic!("Invalid method access");
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::Shape;
    use crate::matrix::Matrix;

    #[test]
    fn test_create_group() {
        let group = Shape::empty_group();
        assert_eq!(group.transformation, Matrix::id44());
        assert!(group.geometry.is_empty());
    }

    #[test]
    fn test_shape_parent() {
        let shape = Shape::test();
        assert!(shape.parent.is_none());
    }
}