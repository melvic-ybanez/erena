use crate::shapes::{Object, Geometry};
use crate::shapes::groups::Group;

#[derive(Debug)]
pub struct Arena<G> {
    objects: Vec<Object<G>>
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct ObjectId {
    pub value: usize
}

impl<G: Clone> Arena<G> {
    pub fn new() -> Arena<G> {
        Arena { objects: vec![] }
    }

    pub fn register(&mut self, node: &Object<G>) -> ObjectId {
        let index = self.objects.len();

        self.objects.push((*node).clone());

        ObjectId { value: index }
    }

    pub fn parent_child(&mut self, parent: &mut Object<G>, child: &mut Object<G>) {
        parent.node.add_child(parent, child, self)
    }

    pub fn read(&mut self, node: &Object<G>) -> ObjectId {
        match node.id {
            None => self.register(node),
            Some(id) => id
        }
    }

    pub fn is_registered(&self, id: usize) -> bool {
        id < self.objects.len()
    }
}