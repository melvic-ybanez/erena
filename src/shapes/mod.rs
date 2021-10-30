use crate::materials::Material;
use crate::matrix::{CanTransform, Matrix};
use crate::rays::{Ray, Intersection3D};
use crate::shapes::Geometry::{Sphere, TestShape, Plane, Cube, Cylinder };
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::math::Real;
use crate::shapes::cylinders::CylLike;
use crate::shapes::groups::Group;

#[derive(Debug, PartialEq, Clone)]
pub struct Object<G> {
    pub transformation: Matrix,
    pub material: Material,
    pub geometry: Group<G>,
    pub parent: Option<Group<G>>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Geometry {
    Sphere,
    TestShape,
    Plane,
    Cube,
    Cylinder(CylLike)
}

pub type Shape = Object<Geometry>;

impl Shape {
    pub fn new(geo: Group<Geometry>) -> Shape {
        Object {
            transformation: Matrix::id44(),
            material: Material::default(),
            geometry: geo,
            parent: None
        }
    }

    pub fn one(geo: Geometry) -> Shape {
        Shape::new(Group::Leaf(geo))
    }

    pub fn sphere() -> Shape {
        Shape::one(Sphere)
    }

    pub fn test() -> Shape {
        Shape::one(TestShape)
    }

    pub fn plane() -> Shape {
        Shape::one(Plane)
    }

    pub fn cube() -> Shape {
        Shape::one(Cube)
    }

    pub fn cylinder() -> Shape {
        CylLike::cylinder().to_shape()
    }

    pub fn cone() -> Shape {
        CylLike::cone().to_shape()
    }

    pub fn group(objects: Vec<Shape>) -> Shape {
        Shape::new(Group::Tree(objects))
    }

    pub fn empty_group() -> Shape {
        Shape::group(vec![])
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection3D> {
        let local_ray = ray.transform(self.transformation.inverse_or_id44());

        match self.geometry {
            Group::Leaf(Sphere) => spheres::intersect(self, &local_ray),
            Group::Leaf(TestShape) => test::intersect(self, &local_ray),
            Group::Leaf(Plane) => planes::intersect(self, &local_ray),
            Group::Leaf(Cube) => cubes::intersect(self, &local_ray),
            Group::Leaf(Cylinder(CylLike { cone, .. })) =>
                cylinders::intersect(self, &local_ray, cone),
            Group::Tree(_) => unimplemented!()
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        let inverse = self.transformation.inverse_or_id44();
        let local_point = &inverse * point;

        let local_normal = match self.geometry {
            Group::Leaf(Sphere) => spheres::normal_at(local_point),
            Group::Leaf(TestShape) => test::normal_at(local_point),
            Group::Leaf(Plane) => planes::normal_at(),
            Group::Leaf(Cube) => cubes::normal_at(local_point),
            Group::Leaf(Cylinder(CylLike { min, max, cone, .. })) =>
                cylinders::normal_at(local_point, min, max, cone),
            Group::Tree(_) => unimplemented!()
        };

        let world_normal = inverse.transpose() * local_normal;
        let world_normal = world_normal.to_vector();     // set w to 0 first
        world_normal.normalize()
    }

    pub fn material_ref(self, material: &Material) -> Shape {
        self.material(material.clone())
    }

    pub fn material(mut self, material: Material) -> Shape {
        self.material = material;
        self
    }

    pub fn geometry(mut self, geometry: Geometry) -> Shape {
        self.geometry = Group::Leaf(geometry);
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
    use crate::shapes::{Shape, Group};
    use crate::shapes::Geometry::TestShape;
    use crate::tuples::points::Point;
    use crate::tuples::vectors::Vector;

    pub static mut SAVED_RAY: Option<Ray> = None;

    pub fn intersect<'a>(shape: &'a Shape, ray: &Ray) -> Vec<Intersection3D<'a>> {
        if let Group::Leaf(TestShape) = shape.geometry {
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
pub mod cylinders;
mod groups;