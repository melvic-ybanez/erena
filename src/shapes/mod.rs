use crate::materials::Material;
use crate::matrix::{CanTransform, Matrix};
use crate::rays::{Ray, Intersection3D};
use crate::shapes::Space3D::{Sphere, TestShape, Plane, Cube, Cylinder };
use crate::tuples::points::Point;
use crate::tuples::vectors::Vector;
use crate::math::Real;

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
    Cylinder {
        min: Real,
        max: Real,
        closed: bool,
        cone: bool,
    },
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
        Shape::new(Space3D::cylinder())
    }

    pub fn cone() -> Shape {
        Shape::new(Space3D::cone())
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection3D> {
        let local_ray = ray.transform(self.transformation.inverse_or_id44());

        match self.shape {
            Sphere => spheres::intersect(self, &local_ray),
            TestShape => test::intersect(self, &local_ray),
            Plane => planes::intersect(self, &local_ray),
            Cube => cubes::intersect(self, &local_ray),
            Cylinder { cone, .. } => cylinders::intersect(self, &local_ray, cone),
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
            Cylinder { min, max, cone, .. } => cylinders::normal_at(local_point, min, max, cone),
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

    pub fn shape(mut self, shape: Space3D) -> Shape {
        self.shape = shape;
        self
    }
}

impl Space3D {
    pub fn cylinder() -> Space3D {
        Space3D::cylinder_like(false)
    }

    pub fn cone() -> Space3D {
        Space3D::cylinder_like(true)
    }

    pub fn cylinder_like(cone: bool) -> Space3D {
        Space3D::Cylinder {
            min: -Real::INFINITY,
            max: Real::INFINITY,
            closed: false,
            cone,
        }
    }

    pub fn min(mut self, new_min: Real) -> Space3D {
        if let Space3D::Cylinder { ref mut min, .. } = self {
            *min = new_min;
        }
        self
    }

    pub fn max(mut self, new_max: Real) -> Self {
        if let Space3D::Cylinder { ref mut max, .. } = self {
            *max = new_max;
        }
        self
    }

    pub fn closed(mut self, new_closed: bool) -> Self {
        if let Space3D::Cylinder { ref mut closed, .. } = self {
            *closed = new_closed;
        }
        self
    }

    pub fn is_cone(&self) -> bool {
        if let Space3D::Cylinder { cone, .. } = self {
            return *cone
        }
        false
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