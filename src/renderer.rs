use std::fs;

use crate::camera::Camera;
use crate::lights::PointLight;
use crate::materials::Material;
use crate::math;
use crate::matrix::{scaling, translation, view_transformation, CanTransform};
use crate::scene::World3D;
use crate::shapes::Shape;
use crate::tuples::{colors, points, vectors};
use crate::tuples::colors::Color;
use crate::patterns::Pattern;
use crate::math::Real;

pub(crate) fn render_scene() {
    let floor = Shape::plane()
        .material(
            Material::default()
                .pattern(Pattern::checkers(Color::white(), colors::new(0.5, 0.5, 0.5)))
        );

    let middle = Shape::sphere()
        .translate(-0.5, 1.0, 0.5)
        .material(
            Material::default()
                .pattern(
                    Pattern::checkers(
                        colors::new(21.0 / 255.0, 184.0 / 255.0, 0.0),
                        colors::new(0.1, 1.0, 0.5),
                    ).scale(0.25, 0.25, 0.25).rotate_y(-math::PI / 4.0)
                )
                .color(colors::new(0.1, 1.0, 0.5))
                .diffuse(0.7)
                .specular(0.3)
        );

    let right = Shape::sphere()
        .transform(translation(1.1, 1.0, 0.7) * scaling(0.5, 0.5, 0.5))
        .material(
            Material::default()
                .color(colors::new(1.0, 0.5, 0.5))
                .diffuse(0.7)
                .specular(0.3)
        );

    let left = Shape::sphere()
        .transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33))
        .material(
            Material::default()
                .pattern(Pattern::ring(colors::new(1.0, 0.8, 0.1), Color::white())
                    .scale(0.33, 0.33, 0.33)
                    .rotate_x(-math::PI / 4.0))
                .diffuse(0.7)
                .specular(0.3)
        );

    let mut small_spheres: Vec<Shape> = vec![];
    for i in 0..5 {
        let component_scale = 0.5 + 0.1 * (i as Real);
        let pattern = Pattern::gradient(
            colors::new(1.0, 0.8, 0.1),
            colors::new(220.0 / 255.0, 20.0 / 255.0, 60.0 / 255.0)
        );
        small_spheres.push(
            left.clone().transform(translation(i as Real, 0.0, 0.0) *
                scaling(component_scale, component_scale, component_scale))
                .material(
                    Material::default()
                        .color(colors::new(0.5, 0.6, 1.0))
                        .diffuse(0.7)
                        .specular(0.3)
                        .pattern_opt(if i % 2 == 0 { Some(pattern) } else { None })
                )
        )
    };

    let mut objects = vec![floor, middle, right, left];
    objects.append(&mut small_spheres);

    let world = World3D::new(
        objects,
        Some(PointLight::new(points::new(-10.0, 10.0, -10.0), Color::white())),
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