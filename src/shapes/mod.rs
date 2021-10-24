use crate::materials::Material;
use crate::matrix::{CanTransform, Matrix};
use crate::rays::{Ray, Intersection3D};
use crate::shapes::Space3D::{Sphere, TestShape};
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
    TestShape,
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

    pub fn test() -> Shape {
        Shape::new(TestShape)
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection3D> {
        let local_ray = ray.transform(self.transformation.inverse_or_id44());

        match self.shape {
            Sphere => sphere::intersect(self, &local_ray),
            TestShape => test::intersect(self, &local_ray)
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        match self.shape {
            Sphere => sphere::normal_at(self, point),
            TestShape => unimplemented!()
        }
    }

    pub fn with_material(mut self, material: Material) -> Shape {
        self.material = material;
        self
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

mod test {
    use crate::rays::{Ray, Intersection3D};
    use crate::shapes::Shape;
    use crate::shapes::Space3D::TestShape;

    pub static mut SAVED_RAY: Option<Ray> = None;

    pub fn intersect<'a>(shape: &'a Shape, ray: &Ray) -> Vec<Intersection3D<'a>> {
        if let TestShape = shape.shape {
            unsafe {
                SAVED_RAY = Some(Ray::new(ray.origin, ray.direction));
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests;

mod sphere;