use crate::lights::PointLight;
use crate::matrix::{CanTransform, scaling};
use crate::rays::{Comps3D, Intersection, Ray, Intersection3D, Comps};
use crate::shapes::{Object, Shape, Space3D};
use crate::tuples::{colors, points};
use crate::tuples::colors::Color;
use crate::tuples::points::Point;

#[derive(Clone)]
pub struct World<S> {
    pub objects: Vec<Object<S>>,
    pub light: Option<PointLight>,
}

pub type World3D = World<Space3D>;

const DEFAULT_DEPTH: u8 = 4;

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

    pub fn add_object(&mut self, object: &Object<S>) where S: Clone {
        self.objects.push((*object).clone());
    }

    pub fn add_objects(&mut self, objects: &Vec<&Object<S>>) where S: Clone {
        for obj in objects {
            self.add_object(obj);
        }
    }

    fn contains(&self, shape: &Object<S>) -> bool where S: PartialEq {
        self.objects.contains(shape)
    }

    pub fn add_light(&mut self, point_light: PointLight) {
        self.light = Some(point_light);
    }

    pub fn update_object<F>(&mut self, i: usize, f: F) where S: Clone, F: Fn(Object<S>) -> (Object<S>) {
        self.objects[i] = f(self.objects[i].clone());
    }
}

impl World3D {
    pub(crate) fn default() -> World3D {
        let mut world = World::empty();

        let mut sphere1 = Shape::sphere();
        sphere1.material.color = colors::new(0.8, 1.0, 0.6);
        sphere1.material.diffuse = 0.7;
        sphere1.material.specular = 0.2;

        let sphere2 = Shape::sphere().transform(scaling(0.5, 0.5, 0.5));

        world.add_object(&sphere1);
        world.add_object(&sphere2);
        world.light = Some(PointLight::new(points::new(-10.0, 10.0, -10.0), Color::white()));
        world
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection<Space3D>> {
        let mut intersections: Vec<Intersection3D> = vec![];
        for obj in self.objects.iter() {
            intersections.append(&mut obj.intersect(ray));
        }
        intersections.sort_by(Intersection::compare);
        intersections
    }

    fn is_shadowed(&self, point: Point) -> bool {
        let light = self.light.expect("Light source is required");

        let v = (light.position - point).to_vector();
        let distance = v.magnitude();
        let direction = v.normalize();

        let ray = Ray::new(point, direction);
        let intersections = self.intersect(&ray);

        match Intersection::hit(intersections) {
            None => false,
            Some(hit) => hit.t < distance
        }
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        self.color_at_with_depth(ray, DEFAULT_DEPTH)
    }

    pub fn color_at_with_depth(&self, ray: &Ray, depth: u8) -> Color {
        if let Some(hit) = Intersection::hit(self.intersect(ray)) {
            let comps = Comps3D::prepare(hit, ray);
            self.shade_hit_with_depth(comps, depth)
        } else {
            Color::black()
        }
    }

    fn shade_hit(&self, comps: Comps3D) -> Color {
        self.shade_hit_with_depth(comps, DEFAULT_DEPTH)
    }

    fn shade_hit_with_depth(&self, comps: Comps3D, depth: u8) -> Color {
        if let Some(light) = self.light {
            let shadowed = self.is_shadowed(comps.get_overpoint());
            let surface = comps.get_object().material.lighting(
                comps.get_object(),
                light,
                comps.get_overpoint(),
                comps.get_eye_vec(),
                comps.get_normal_vec(),
                shadowed,
            );
            let reflected = self.reflected_color_with_depth(comps, depth);
            surface + reflected
        } else {
            Color::black()
        }
    }

    pub fn reflected_color(&self, comps: Comps3D) -> Color {
        self.reflected_color_with_depth(comps, DEFAULT_DEPTH)
    }

    pub fn reflected_color_with_depth(&self, comps: Comps3D, depth: u8) -> Color {
        if depth == 0 || comps.get_object().material.reflective == 0.0 {
            Color::black()
        } else {
            let reflect_ray = Ray::new(comps.get_overpoint(), comps.get_reflect_vec());
            let color = self.color_at_with_depth(&reflect_ray, depth - 1);
            color * comps.get_object().material.reflective
        }
    }
}

#[cfg(test)]
mod tests;
