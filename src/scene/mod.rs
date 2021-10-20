use crate::shapes::{Shape, Object, Space3D};
use crate::tuples::{colors, points};
use crate::matrix::scaling;
use crate::lights::PointLight;
use crate::tuples::colors::Color;
use crate::rays::{Ray, Intersection, Comps3D};

pub struct World<S> {
    pub objects: Vec<Object<S>>,
    pub light: Option<PointLight>,
}

pub type World3D = World<Space3D>;

impl<S> World<S> {
    fn new() -> World<S> {
        World { objects: vec![], light: None }
    }

    fn is_empty(&self) -> bool {
        self.objects.is_empty() && self.light.is_none()
    }

    fn add_shape(&mut self, object: Object<S>) {
        self.objects.push(object);
    }

    fn contains(&self, shape: Object<S>) -> bool where S: PartialEq {
        self.objects.contains(&shape)
    }
}

impl World3D {
    fn default() -> World3D {
        let mut world = World::new();

        let mut sphere1 = Shape::sphere();
        sphere1.material.color = colors::new(0.8, 1.0, 0.6);
        sphere1.material.diffuse = 0.7;
        sphere1.material.specular = 0.2;

        let mut sphere2 = Shape::sphere();
        sphere2.transform(scaling(0.5, 0.5, 0.5));

        world.add_shape(sphere1);
        world.add_shape(sphere2);
        world.light = Some(PointLight::new(points::new(-10.0, 10.0, -10.0), Color::white()));
        world
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection<Space3D>> {
        let mut intersections: Vec<_> = self.objects
            .iter()
            .map(|obj| obj.intersect(ray))
            .flatten()
            .collect();
        intersections.sort_by(Intersection::compare);
        intersections
    }

    fn shade_hit(&self, comps: Comps3D) -> Color {
        match self.light {
            None => Color::black(),
            Some(light) =>
                comps.object.material.lighting(light, comps.point, comps.eye_vec, comps.normal_vec)
        }
    }
}

#[cfg(test)]
mod tests;
