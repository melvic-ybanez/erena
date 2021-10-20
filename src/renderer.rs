use crate::shapes::Shape;
use crate::materials::Material;
use crate::tuples::{colors, points};
use crate::tuples::colors::Color;
use crate::lights::PointLight;
use crate::canvas::Canvas;
use crate::rays::{Ray, Intersection};
use std::fs;

pub(crate) fn render_scene() {
    let ray_origin = points::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 200;
    let pixel_size = wall_size / (canvas_pixels as f64);
    let half = wall_size / 2.0;

    let mut sphere = Shape::sphere();
    sphere.material = Material::default();
    sphere.material.color = colors::new(1.0, 0.2, 0.1);

    let light_position = points::new(-10.0, 10.0, -10.0);
    let light_color = Color::white();
    let light = PointLight::new(light_position, light_color);

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);
            let position = points::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).to_vector().normalize());
            let xs = sphere.intersect(&ray);

            match Intersection::hit(xs) {
                None => (),
                Some(hit) => {
                    let point = ray.position(hit.t);
                    let normal = hit.object.normal_at(point);
                    let eye = -ray.direction;
                    canvas[(x, y)] = hit.object.material.lighting(light, point, eye, normal);
                }
            }
        }
    };

    fs::write("erena.ppm", canvas.to_ppm().to_string()).expect("Can not render scene")
}