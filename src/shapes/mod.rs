use crate::materials::Material;
use crate::matrix::{CanTransform, Matrix};
use crate::rays::{Ray, Intersection3D};
use crate::shapes::Geometry::{Sphere, TestShape, Plane, Cube, Cylinder };
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::math::Real;

#[derive(Debug, PartialEq, Clone)]
pub struct Object<G> {
    pub transformation: Matrix,
    pub material: Material,
    pub geometry: GeoType<G>,
    pub parent: Option<Group<G>>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Geometry {
    Sphere,
    TestShape,
    Plane,
    Cube,
    Cylinder {
        min: Real,
        max: Real,
        closed: bool,
        cone: bool,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum GeoType<S> {
    One(S),
    Many(Group<S>)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Group<S> {
    pub objects: Vec<Object<S>>
}

pub type Shape = Object<Geometry>;

impl Shape {
    pub fn new(geo: GeoType<Geometry>) -> Shape {
        Object {
            transformation: Matrix::id44(),
            material: Material::default(),
            geometry: geo,
            parent: None
        }
    }

    pub fn one(geo: Geometry) -> Shape {
        Shape::new(GeoType::One(geo))
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
        Shape::one(Geometry::cylinder())
    }

    pub fn cone() -> Shape {
        Shape::one(Geometry::cone())
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection3D> {
        let local_ray = ray.transform(self.transformation.inverse_or_id44());

        match self.geometry {
            GeoType::One(Sphere) => spheres::intersect(self, &local_ray),
            GeoType::One(TestShape) => test::intersect(self, &local_ray),
            GeoType::One(Plane) => planes::intersect(self, &local_ray),
            GeoType::One(Cube) => cubes::intersect(self, &local_ray),
            GeoType::One(Cylinder { cone, .. }) => cylinders::intersect(self, &local_ray, cone),
            GeoType::Many(_) => unimplemented!()
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        let inverse = self.transformation.inverse_or_id44();
        let local_point = &inverse * point;

        let local_normal = match self.geometry {
            GeoType::One(Sphere) => spheres::normal_at(local_point),
            GeoType::One(TestShape) => test::normal_at(local_point),
            GeoType::One(Plane) => planes::normal_at(),
            GeoType::One(Cube) => cubes::normal_at(local_point),
            GeoType::One(Cylinder { min, max, cone, .. }) =>
                cylinders::normal_at(local_point, min, max, cone),
            GeoType::Many(_) => unimplemented!()
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
        self.geometry = GeoType::One(geometry);
        self
    }
}

impl Geometry {
    pub fn cylinder() -> Geometry {
        Geometry::cylinder_like(false)
    }

    pub fn cone() -> Geometry {
        Geometry::cylinder_like(true)
    }

    pub fn cylinder_like(cone: bool) -> Geometry {
        Geometry::Cylinder {
            min: -Real::INFINITY,
            max: Real::INFINITY,
            closed: false,
            cone,
        }
    }

    pub fn min(mut self, new_min: Real) -> Geometry {
        if let Geometry::Cylinder { ref mut min, .. } = self {
            *min = new_min;
        }
        self
    }

    pub fn max(mut self, new_max: Real) -> Self {
        if let Geometry::Cylinder { ref mut max, .. } = self {
            *max = new_max;
        }
        self
    }

    pub fn closed(mut self, new_closed: bool) -> Self {
        if let Geometry::Cylinder { ref mut closed, .. } = self {
            *closed = new_closed;
        }
        self
    }

    pub fn is_cone(&self) -> bool {
        if let Geometry::Cylinder { cone, .. } = self {
            return *cone
        }
        false
    }
}

impl GeoType<Geometry> {
    pub fn is_cone(&self) -> bool {
        if let GeoType::One(geometry) = self {
            return geometry.is_cone()
        }
        return false
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
    use crate::shapes::{Shape, GeoType};
    use crate::shapes::Geometry::TestShape;
    use crate::tuples::points::Point;
    use crate::tuples::vectors::Vector;

    pub static mut SAVED_RAY: Option<Ray> = None;

    pub fn intersect<'a>(shape: &'a Shape, ray: &Ray) -> Vec<Intersection3D<'a>> {
        if let GeoType::One(TestShape) = shape.geometry {
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
mod groups;