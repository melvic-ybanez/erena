use crate::rays::{Ray, Intersection};
use crate::tuples::points::Point;
use crate::matrix::Matrix;
use crate::tuples::vectors::Vector;
use crate::materials::Material;
use crate::shapes::Space3D::Sphere;

#[derive(Debug, PartialEq, Clone)]
pub struct Object<S> {
    pub transformation: Matrix,
    pub material: Material,
    pub shape: S,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Space3D {
    Sphere,
}

pub type Shape = Object<Space3D>;

impl Shape {
    pub fn new(space3d: Space3D) -> Shape {
        Object {
            transformation: Matrix::id44(),
            material: Material::default(),
            shape: space3d
        }
    }

    pub fn sphere() -> Shape {
        Shape::new(Sphere)
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection<Space3D>> {
        match self.shape {
            Sphere => sphere::intersect(self, ray)
        }
    }

    pub fn transform(&mut self, transformation: Matrix) -> &Self {
        match self.shape {
            Sphere => sphere::transform(self, transformation)
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        match self.shape {
            Sphere => sphere::normal_at(self, point)
        }
    }

    pub fn with_material(&mut self, material: Material) -> &Self {
        match self.shape {
            Sphere => sphere::with_material(self, material)
        }
    }
}

mod sphere {
    use crate::shapes::{Shape, Space3D};
    use crate::rays::{Ray, Intersection};
    use crate::tuples::points::Point;
    use crate::matrix::Matrix;
    use crate::tuples::vectors::Vector;
    use crate::materials::Material;

    pub fn intersect<'a>(sphere: &'a Shape, ray: &Ray) -> Vec<Intersection<'a, Space3D>> {
        let transformation = sphere.transformation.inverse_or_id44();
        let ray = ray.transform(&transformation);

        // note: sphere's center is at world origin
        let sphere_to_ray = ray.origin - Point::origin();

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            vec![
                Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), sphere),     // t1
                Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), sphere),     // t2
            ]
        }
    }

    pub fn transform(sphere: &mut Shape, transformation: Matrix) -> &Shape {
        sphere.transformation = transformation;
        sphere
    }

    pub fn normal_at(sphere: &Shape, world_point: Point) -> Vector {
        let inverse = sphere.transformation.inverse_or_id44();
        let object_point = &inverse * world_point;
        let object_normal = object_point - Point::origin();
        let world_normal = inverse.transpose() * object_normal;
        world_normal.to_vector().normalize()
    }

    pub fn with_material(sphere: &mut Shape, material: Material) -> &Shape {
        sphere.material = material;
        sphere
    }
}

#[cfg(test)]
mod tests;