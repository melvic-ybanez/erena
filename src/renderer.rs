use crate::shapes::Shape;
use crate::materials::Material;
use crate::tuples::{colors, points, vectors};
use crate::tuples::colors::Color;
use crate::lights::PointLight;
use crate::canvas::Canvas;
use crate::rays::{Ray, Intersection};
use std::fs;
use crate::matrix::{scaling, translation, rotation_y, rotation_x, view_transformation};
use crate::math;
use crate::scene::World3D;
use crate::camera::Camera;

pub(crate) fn render_scene() {
    let mut floor = Shape::sphere();
    floor.transformation = scaling(10.0, 0.01, 10.0);
    floor.material = Material::default();
    floor.material.color = colors::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Shape::sphere();
    left_wall.transformation = translation(0.0, 0.0, 5.0)
        * rotation_y(-math::PI / 4.0)
        * rotation_x(math::PI / 2.0)
        * scaling(10.0, 0.01, 10.0);
    left_wall.material = floor.material;

    let mut right_wall = Shape::sphere();
    right_wall.transformation = translation(0.0, 0.0, 5.0)
        * rotation_y(math::PI / 4.0)
        * rotation_x(math::PI / 2.0)
        * scaling(10.0, 0.01, 10.0);
    right_wall.material = floor.material;

    let mut middle = Shape::sphere();
    middle.transformation = translation(-0.5, 1.0, 0.5);
    middle.material = Material::default();
    middle.material.color = colors::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Shape::sphere();
    right.transformation = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material = Material::default();
    right.material.color = colors::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Shape::sphere();
    left.transformation = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material = Material::default();
    left.material.color = colors::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World3D::new(
        vec![floor, left_wall, middle, right_wall, right, left],
        Some(PointLight::new(points::new(-10.0, 10.0, -10.0), Color::white()))
    );

    let mut camera = Camera::new(200, 100, math::PI / 3.0);
    camera.transformation = view_transformation(
        points::new(0.0, 1.5, -5.0),
        points::new(0.0, 1.0, 0.0),
        vectors::new(0.0, 1.0, 0.0)
    );

    let canvas = camera.render(world);

    fs::write("erena.ppm", canvas.to_ppm().to_string()).expect("Can not render scene");
}