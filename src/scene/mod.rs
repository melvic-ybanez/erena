use crate::shapes::{Shape, Object, Sphere};
use crate::tuples::{colors, points};
use crate::matrix::scaling;
use crate::lights::PointLight;
use crate::tuples::colors::Color;

pub struct World {
    pub objects: Vec<Box<dyn Object>>,
    pub light: Option<PointLight>,
}

impl World {
    fn new() -> World {
        World { objects: vec![], light: None }
    }

    fn is_empty(&self) -> bool {
        self.objects.is_empty() && self.light.is_none()
    }

    fn add_shape(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    fn default() -> World {
        let mut world = World::new();

        let mut sphere1 = Sphere::new();
        sphere1.material.color = colors::new(0.8, 1.0, 0.6);
        sphere1.material.diffuse = 0.7;
        sphere1.material.specular = 0.2;

        let mut sphere2 = Sphere::new();
        sphere2.transform(scaling(0.5, 0.5, 0.5));

        world.add_shape(Box::new(sphere1));
        world.add_shape(Box::new(sphere2));
        world.light = Some(PointLight::new(points::new(-10.0, 10.0, -10.0), Color::white()));
        world
    }

    fn contains(&self, shape: Box<dyn Object>) -> bool {
        self.objects.contains(&shape)
    }
}

#[cfg(test)]
mod tests;
