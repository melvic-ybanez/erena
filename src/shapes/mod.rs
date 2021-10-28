use crate::materials::Material;
use crate::matrix::{CanTransform, Matrix};
use crate::rays::{Ray, Intersection3D};
use crate::shapes::Space3D::{Sphere, TestShape, Plane, Cube, Cylinder};
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
    Plane,
    Cube,
    Cylinder,
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

    pub fn plane() -> Shape {
        Shape::new(Plane)
    }

    pub fn cube() -> Shape {
        Shape::new(Cube)
    }

    pub fn cylinder() -> Shape {
        Shape::new(Cylinder)
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection3D> {
        let local_ray = ray.transform(self.transformation.inverse_or_id44());

        match self.shape {
            Sphere => spheres::intersect(self, &local_ray),
            TestShape => test::intersect(self, &local_ray),
            Plane => planes::intersect(self, &local_ray),
            Cube => cubes::intersect(self, &local_ray),
            Cylinder => cylinders::intersect(self, &local_ray),
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        let inverse = self.transformation.inverse_or_id44();
        let local_point = &inverse * point;

        let local_normal = match self.shape {
            Sphere => spheres::normal_at(local_point),
            TestShape => test::normal_at(local_point),
            Plane => planes::normal_at(),
            Cube => cubes::normal_at(local_point),
            Cylinder => unimplemented!()
        };

        let world_normal = inverse.transpose() * local_normal;
        world_normal.to_vector().normalize()
    }

    pub fn material_ref(self, material: &Material) -> Shape {
        self.material(material.clone())
    }

    pub fn material(mut self, material: Material) -> Shape {
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
    use crate::tuples::points::Point;
    use crate::tuples::vectors::Vector;

    pub static mut SAVED_RAY: Option<Ray> = None;

    pub fn intersect<'a>(shape: &'a Shape, ray: &Ray) -> Vec<Intersection3D<'a>> {
        if let TestShape = shape.shape {
            unsafe {
                SAVED_RAY = Some(Ray::new(ray.origin, ray.direction));
            }
        }

        vec![]
    }

    pub fn normal_at(local_point: Point) -> Vector {
        local_point.to_vector()
    }
}

#[cfg(test)]
mod tests;

pub mod spheres;
mod planes;
mod cubes;
mod cylinders;