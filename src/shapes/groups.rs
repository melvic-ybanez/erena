use crate::shapes::arena::ObjectId;
use crate::shapes::Shape;

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    pub objects: Vec<ObjectId>
}

impl Group {
    pub fn new(objects: Vec<ObjectId>) -> Group {
        Group { objects }
    }

    pub fn empty() -> Group {
        Group { objects: vec![] }
    }

    pub fn contains(&self, shape: &Shape) -> bool {
        match shape.id {
            None => false,
            Some(ref id) => self.objects.contains(id)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    pub fn non_empty(&self) -> bool {
        !self.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use crate::shapes::{Geo, Shape};
    use crate::shapes::arena::Arena;

    #[test]
    fn test_create_group() {
        let group = Shape::empty_group();
        assert_eq!(group.transformation, Matrix::id44());
        if let Geo::Group(group) = group.geo {
            assert!(group.is_empty());
        } else {
            panic!("Not a group");
        }
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

        arena.connect(&mut group, &mut shape);

        if let Geo::Group(group) = group.geo {
            assert!(group.non_empty());
            assert!(group.contains(&shape));
        } else {
            panic!("Not a group");
        }
    }
}