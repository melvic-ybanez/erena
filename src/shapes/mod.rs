use crate::materials::Material;
use crate::matrix::{CanTransform, Matrix};
use crate::rays::{Ray, Intersection3D};
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::shapes::cylinders::CylLike;
use crate::shapes::groups::Group;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use crate::shapes::bounds::Bounds;
use crate::shapes::triangles::Triangle;

#[derive(Debug, Clone)]
pub struct Object<G> {
    pub transformation: Matrix,
    pub material: Material,
    pub geo: G,
    pub parent: RefCell<Weak<Object<G>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Geo {
    Sphere,
    TestShape,
    Plane,
    Cube,
    Cylinder(CylLike),
    Group(Group),
    Triangle(Triangle)
}

pub type Shape = Object<Geo>;

impl<G> Object<G> {
    pub fn set_parent(&self, parent: Weak<Object<G>>) {
        *self.parent.borrow_mut() = parent;
    }

    pub fn world_to_object(&self, world_point: Point) -> Point {
        let point = match self.get_parent() {
            None => world_point,
            Some(parent) => parent.world_to_object(world_point)
        };
        self.transformation.inverse_or_id44() * point
    }

    pub fn normal_to_world(&self, normal: Vector) -> Vector {
        let normal = (self.transformation.inverse_or_id44().transpose() * normal)
            .to_vector()
            .normalize();
        match self.get_parent() {
            None => normal,
            Some(parent) => parent.normal_to_world(normal)
        }
    }

    pub fn get_parent(&self) -> Option<Rc<Object<G>>> {
        self.parent.borrow().upgrade()
    }
}

impl Shape {
    pub fn new(geo: Geo) -> Shape {
        Object {
            transformation: Matrix::id44(),
            material: Material::default(),
            geo,
            parent: RefCell::new(Weak::new()),
        }
    }

    pub fn sphere() -> Shape {
        Shape::new(Geo::Sphere)
    }

    pub fn test() -> Shape {
        Shape::new(Geo::TestShape)
    }

    pub fn plane() -> Shape {
        Shape::new(Geo::Plane)
    }

    pub fn cube() -> Shape {
        Shape::new(Geo::Cube)
    }

    pub fn cylinder() -> Shape {
        CylLike::cylinder().to_shape()
    }

    pub fn cone() -> Shape {
        CylLike::cone().to_shape()
    }

    pub fn group(objects: Vec<Shape>) -> Shape {
        let objects: Vec<_> = objects.into_iter()
            .map(|obj| Rc::new(obj)).collect();
        Shape::new(Geo::Group(Group::new(objects)))
    }

    pub fn triangle(p1: Point, p2: Point, p3: Point) -> Shape {
        Shape::new(Geo::Triangle(Triangle::new(p1, p2, p3)))
    }

    pub fn empty_group() -> Shape {
        Shape::group(vec![])
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection3D> {
        let local_ray = ray.transform(self.transformation.inverse_or_id44());

        match self.geo {
            Geo::Sphere => spheres::intersect(self, &local_ray),
            Geo::TestShape => test::intersect(self, &local_ray),
            Geo::Plane => planes::intersect(self, &local_ray),
            Geo::Cube => cubes::intersect(self, &local_ray),
            Geo::Cylinder(CylLike { cone, .. }) =>
                cylinders::intersect(self, &local_ray, cone),
            Geo::Group(_) => groups::intersect(self, &local_ray),
            Geo::Triangle(_) => unimplemented!()
        }
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        let local_point = self.world_to_object(world_point);

        let local_normal = match self.geo {
            Geo::Sphere => spheres::normal_at(local_point),
            Geo::TestShape => test::normal_at(local_point),
            Geo::Plane => planes::normal_at(),
            Geo::Cube => cubes::normal_at(local_point),
            Geo::Cylinder(CylLike { min, max, cone, .. }) =>
                cylinders::normal_at(local_point, min, max, cone),
            Geo::Group(_) => groups::normal_at(),
            Geo::Triangle(ref tri) => tri.get_normal()
        };

        self.normal_to_world(local_normal)
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::of(self)
    }

    pub fn material_ref(self, material: &Material) -> Shape {
        self.material(material.clone())
    }

    pub fn material(mut self, material: Material) -> Shape {
        self.material = material;
        self
    }

    pub fn geometry(mut self, geometry: Geo) -> Shape {
        self.geo = geometry;
        self
    }
}

impl<G: PartialEq> PartialEq for Object<G> {
    fn eq(&self, other: &Self) -> bool {
        self.transformation == other.transformation &&
            self.material == other.material &&
            self.geo == other.geo
    }
}

impl Geo {
    pub fn is_cone(&self) -> bool {
        if let Geo::Cylinder(cyl @ CylLike { .. }) = self {
            return cyl.is_cone();
        }
        return false;
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
    use crate::shapes::{Shape, Geo};
    use crate::tuples::points::Point;
    use crate::tuples::vectors::Vector;

    pub static mut SAVED_RAY: Option<Ray> = None;

    pub fn intersect(shape: &Shape, ray: &Ray) -> Vec<Intersection3D> {
        if let Geo::TestShape = shape.geo {
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
mod bounds;
mod triangles;