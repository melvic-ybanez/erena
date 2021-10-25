use std::fs;

use crate::camera::Camera;
use crate::lights::PointLight;
use crate::materials::Material;
use crate::math;
use crate::matrix::{rotation_x, rotation_y, scaling, translation, view_transformation};
use crate::scene::World3D;
use crate::shapes::Shape;
use crate::tuples::{colors, points, vectors};
use crate::tuples::colors::Color;

pub(crate) fn render_scene() {
    let floor = Shape::plane();

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

    let world = World3D::new(
        vec![floor, middle, right, left],
        Some(PointLight::new(points::new(-10.0, 10.0, -10.0), Color::white()))
    );

    let mut camera = Camera::new(200, 100, math::PI / 3.0);
    camera.transformation = view_transformation(
        points::new(0.0, 1.5, -5.0),
        points::new(0.0, 1.0, 0.0),
        vectors::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(world);

    fs::write("erena.ppm", canvas.to_ppm().to_string()).expect("Can not render scene");
}