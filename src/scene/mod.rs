use crate::shapes::{Shape, Object, Space3D};
use crate::tuples::{colors, points};
use crate::matrix::scaling;
use crate::lights::PointLight;
use crate::tuples::colors::Color;
use crate::rays::{Ray, Intersection, Comps3D};

#[derive(Clone)]
pub struct World<S> {
    pub objects: Vec<Object<S>>,
    pub light: Option<PointLight>,
}

pub type World3D = World<Space3D>;

impl<S> World<S> {
    pub fn new(objects: Vec<Object<S>>, light: Option<PointLight>) -> World<S> {
        World { objects, light }
    }

    pub fn empty() -> World<S> {
        World::new(vec![], None)
    }

    fn is_empty(&self) -> bool {
        self.objects.is_empty() && self.light.is_none()
    }

    pub fn add_object(&mut self, object: Object<S>) {
        self.objects.push(object);
    }

    pub fn add_objects(&mut self, objects: &mut Vec<Object<S>>) {
        self.objects.append(objects);
    }

    fn contains(&self, shape: Object<S>) -> bool where S: PartialEq {
        self.objects.contains(&shape)
    }

    fn shade_hit(&self, comps: Comps3D) -> Color {
        match self.light {
            None => Color::black(),
            Some(light) =>
                comps.object.material.lighting(light, comps.point, comps.eye_vec, comps.normal_vec, false)
        }
    }
}

impl World3D {
    pub(crate) fn default() -> World3D {
        let mut world = World::empty();

        let mut sphere1 = Shape::sphere();
        sphere1.material.color = colors::new(0.8, 1.0, 0.6);
        sphere1.material.diffuse = 0.7;
        sphere1.material.specular = 0.2;

        let mut sphere2 = Shape::sphere();
        sphere2.transform(scaling(0.5, 0.5, 0.5));

        world.add_object(sphere1);
        world.add_object(sphere2);
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

    pub(crate) fn color_at(&self, ray: &Ray) -> Color {
        match Intersection::hit(self.intersect(ray)) {
            None => Color::black(),
            Some(hit) => {
                let comps = Comps3D::prepare(hit, ray);
                self.shade_hit(comps)
            }
        }
    }
}

#[cfg(test)]
mod tests;
