use crate::materials::Material;
use crate::matrix::{CanTransform, Matrix};
use crate::rays::{Ray, Intersection3D};
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::shapes::cylinders::CylLike;
use crate::shapes::arena::{ObjectId, GeoArena};
use crate::shapes::groups::Group;

#[derive(Debug, PartialEq, Clone)]
pub struct Object<G> {
    pub id: Option<ObjectId>,
    pub transformation: Matrix,
    pub material: Material,
    pub geo: G,
    pub parent: Option<ObjectId>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Geo {
    Sphere,
    TestShape,
    Plane,
    Cube,
    Cylinder(CylLike),
    Group(Group)
}

pub type Shape = Object<Geo>;

impl<G> Object<G> {
    pub fn set_parent(&mut self, parent: ObjectId) {
        self.parent = Some(parent);
    }

    pub fn set_id(&mut self, id: ObjectId) {
        self.id = Some(id);
    }
}

impl Shape {
    pub fn new(geo: Geo) -> Shape {
        Object {
            id: None,
            transformation: Matrix::id44(),
            material: Material::default(),
            geo,
            parent: None,
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

    pub fn group(objects: Vec<ObjectId>) -> Shape {
        Shape::new(Geo::Group(Group::new(objects)))
    }

    pub fn empty_group() -> Shape {
        Shape::group(vec![])
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection3D> {
        self.intersect_with_arena(ray, &GeoArena::new())
    }

    pub fn intersect_with_arena(&self, ray: &Ray, arena: &GeoArena) -> Vec<Intersection3D> {
        let local_ray = ray.transform(self.transformation.inverse_or_id44());

        match self.geo {
            Geo::Sphere => spheres::intersect(self, &local_ray),
            Geo::TestShape => test::intersect(self, &local_ray),
            Geo::Plane => planes::intersect(self, &local_ray),
            Geo::Cube => cubes::intersect(self, &local_ray),
            Geo::Cylinder(CylLike { cone, .. }) =>
                cylinders::intersect(self, &local_ray, cone),
            Geo::Group(_) => groups::intersect(self, &local_ray, arena)
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        let inverse = self.transformation.inverse_or_id44();
        let local_point = &inverse * point;

        let local_normal = match self.geo {
            Geo::Sphere => spheres::normal_at(local_point),
            Geo::TestShape => test::normal_at(local_point),
            Geo::Plane => planes::normal_at(),
            Geo::Cube => cubes::normal_at(local_point),
            Geo::Cylinder(CylLike { min, max, cone, .. }) =>
                cylinders::normal_at(local_point, min, max, cone),
            Geo::Group(_) => unimplemented!()
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

    pub fn geometry(mut self, geometry: Geo) -> Shape {
        self.geo = geometry;
        self
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

    pub fn intersect<'a>(shape: &'a Shape, ray: &Ray) -> Vec<Intersection3D<'a>> {
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
mod arena;