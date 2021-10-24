use crate::materials::Material;
use crate::matrix::{CanTransform, Matrix};
use crate::rays::{Intersection, Ray};
use crate::shapes::Space3D::Sphere;
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;

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
    use crate::materials::Material;
    use crate::rays::{Intersection, Ray};
    use crate::shapes::{Shape, Space3D};
    use crate::tuples::points::Point;
    use crate::tuples::vectors::Vector;

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

impl<S> CanTransform for Object<S> {
    fn get_transformation(&self) -> &Matrix {
        &self.transformation
    }

    fn set_transformation(mut self, transformation: Matrix) -> Self {
        self.transformation = transformation;
        self
    }
}

#[cfg(test)]
mod tests;