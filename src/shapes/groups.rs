use crate::shapes::arena::{ObjectId, GeoArena};
use crate::shapes::{Shape, Geo};
use crate::rays::{Ray, Intersection3D, Intersection};

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

pub fn intersect<'a>(shape: &'a Shape, ray: &Ray, arena: &GeoArena) -> Vec<Intersection3D<'a>> {
    if let Geo::Group(Group { ref objects }) = shape.geo {
        let mut xs: Vec<_> = vec![];
        let objects: Vec<_> = objects.iter().map(|id| arena.read_object(*id)).collect();

        for object in objects.iter() {
            xs.append(&mut object.intersect_with_arena(ray, arena));
        }
        xs.sort_by(Intersection::compare);
    }
    vec![]
}

pub fn not_a_group() {
    panic!("Not a group");
}