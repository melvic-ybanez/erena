use crate::shapes::{Object, Geometry, Shape};
use crate::shapes::Geometry::Cylinder;
use crate::shapes::cylinders::CylLike;
use crate::shapes::arena::{ObjectId, Arena};

#[derive(Debug, PartialEq, Clone)]
pub enum Group<G> {
    Leaf(G),
    Tree(Vec<ObjectId>)
}

type Group3D = Group<Geometry>;

impl<G> Group<G> {
    pub fn empty() -> Group<G> {
        Group::Tree(vec![])
    }

    pub fn contains(&self, shape: &Object<G>) -> bool where G: PartialEq {
        if let Group::Tree(objects) = self {
            return match shape.id {
                None => false,
                Some(ref id) => objects.contains(id)
            }
        }
        false
    }

    pub fn add_child(&self, parent: &mut Object<G>, child: &mut Object<G>, arena: &mut Arena<G>)
        where G: Clone {
        if let Group::Tree(mut objects) = self {
            objects.push(arena.read(child));
            child.set_parent(arena.read(parent));
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Group::Tree(objects) = self {
            return objects.is_empty()
        }
        panic!("Invalid method access");
    }

    pub fn non_empty(&self) -> bool {
        !self.is_empty()
    }
}

impl Group3D {
    pub fn is_cone(&self) -> bool {
        if let Group::Leaf(Cylinder(cyl @ CylLike { .. })) = self {
            return cyl.is_cone()
        }
        return false
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::Shape;
    use crate::matrix::Matrix;
    use crate::shapes::groups::Group3D;
    use crate::shapes::arena::Arena;

    #[test]
    fn test_create_group() {
        let group = Shape::empty_group();
        assert_eq!(group.transformation, Matrix::id44());
        assert!(group.node.is_empty());
    }

    #[test]
    fn test_shape_parent() {
        let shape = Shape::test();
        assert!(shape.parent.is_none());
    }

    #[test]
    fn test_add_child() {
        let mut arena = Arena::new();
        let mut group = Shape::empty_group();
        let mut shape = Shape::test();

        arena.parent_child(&mut group, &mut shape);

        assert!(group.node.non_empty());
        assert!(group.node.contains(&shape));
    }
}