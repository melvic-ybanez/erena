use crate::shapes::{Object, Geo, Shape};

#[derive(Debug)]
pub struct Arena<G> {
    objects: Vec<Object<G>>
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct ObjectId {
    pub value: usize
}

impl ObjectId {
    pub fn new(value: usize) -> ObjectId {
        ObjectId { value }
    }
}

pub type GeoArena = Arena<Geo>;

impl GeoArena {
    pub fn new() -> GeoArena {
        Arena { objects: vec![] }
    }

    pub fn register(&mut self, node: &mut Shape) -> ObjectId {
        let index = self.objects.len();
        let id = ObjectId::new(index);

        node.set_id(id);
        self.objects.push((*node).clone());

        id
    }

    pub fn connect(&mut self, parent: &mut Shape, child: &mut Shape) {
        if let Geo::Group(ref mut group) = parent.geo {
            group.objects.push(self.read(child));
            child.set_parent(self.read(parent));
        }
    }

    pub fn read(&mut self, node: &mut Shape) -> ObjectId {
        match node.id {
            None => self.register(node),
            Some(id) => id
        }
    }

    pub fn is_registered(&self, id: usize) -> bool {
        id < self.objects.len()
    }
}