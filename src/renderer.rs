use std::fs;

use crate::materials::Material;
use crate::math;
use crate::math::random::RandGen;
use crate::math::Real;
use crate::matrix::{scaling, translation, view_transformation, CanTransform};
use crate::patterns::Pattern;
use crate::rays::lights::{AreaLight, PointLight};
use crate::scene::camera::Camera;
use crate::scene::World3D;
use crate::shapes::cylinders::CylLike;
use crate::shapes::{Geo, Shape};
use crate::tuples::colors::Color;
use crate::tuples::points::Point;
use crate::tuples::{colors, points, vectors};
use std::rc::Rc;

pub(crate) fn render_scene() {
    let floor = Shape::plane().material(
        Material::default()
            .pattern(Pattern::checkers(
                Color::white(),
                colors::new(0.5, 0.5, 0.5),
            ))
            .reflective(0.2),
    );

    let mut objects = vec![floor, middle()];
    objects.append(&mut cones());

    let full_vec = vectors::new(-10.0, 12.0, -10.0);
    let mut world = World3D::new(
        objects,
        Some(AreaLight::new(
            Point::origin(),
            full_vec,
            4,
            full_vec,
            4,
            Color::white(),
            RandGen::Live,
        )),
    );
    world.add_objects_refs(vec![
        &right(),
        &bottom(),
        &cylinders(),
        &glasses(),
        &triangles(),
    ]);

    let mut camera = Camera::new(1000, 600, math::PI / 3.0);
    camera.transformation = view_transformation(
        points::new(0.0, 1.5, -5.0),
        points::new(0.0, 1.0, 0.0),
        vectors::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(world, true);

    fs::write("erena.ppm", canvas.to_ppm().to_string()).expect("Can not render scene");
}

fn middle() -> Shape {
    Shape::sphere().translate(-0.5, 1.0, 0.5).material(
        Material::default()
            .pattern(
                Pattern::checkers(
                    colors::new(21.0 / 255.0, 184.0 / 255.0, 0.0),
                    colors::new(0.1, 1.0, 0.5),
                )
                .scale(0.25, 0.25, 0.25)
                .rotate_y(-math::PI / 4.0),
            )
            .color(colors::new(0.1, 1.0, 0.5))
            .diffuse(0.7)
            .specular(0.3)
            .reflective(0.5),
    )
}

fn right() -> Rc<Shape> {
    let right_sphere = Shape::sphere()
        .transform(translation(1.1, 2.1, 3.0) * scaling(0.7, 0.7, 0.7))
        .material(
            Material::default()
                .color(colors::new(1.0, 0.5, 0.5))
                .diffuse(0.7)
                .specular(0.3)
                .reflective(0.5),
        );

    let cube = Shape::cube()
        .scale(0.7, 0.7, 0.7)
        .rotate_y(math::PI / 4.0)
        .translate(1.1, 0.7, 3.0)
        .material(
            Material::default().diffuse(0.7).specular(0.3).pattern(
                Pattern::checkers(colors::new(1.0, 0.8, 0.1), Color::white())
                    .scale(0.33, 0.33, 0.33)
                    .rotate_x(-math::PI / 4.0),
            ),
        );

    let group = Rc::new(Shape::empty_group());
    if let Geo::Group(g) = &group.geo {
        g.add_children(
            Rc::downgrade(&group),
            vec![Rc::new(right_sphere), Rc::new(cube)],
        );
    }
    group
}

fn bottom() -> Rc<Shape> {
    let left = Shape::sphere()
        .transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33))
        .material(
            Material::default()
                .pattern(
                    Pattern::ring(colors::new(1.0, 0.8, 0.1), Color::white())
                        .scale(0.33, 0.33, 0.33)
                        .rotate_x(-math::PI / 4.0),
                )
                .diffuse(0.7)
                .specular(0.3)
                .reflective(0.5),
        );

    let mut small_spheres: Vec<Shape> = vec![];
    for i in 0..5 {
        let component_scale = 0.5 + 0.1 * (i as Real);
        let pattern = Pattern::gradient(
            colors::new(1.0, 0.8, 0.1),
            colors::new(220.0 / 255.0, 20.0 / 255.0, 60.0 / 255.0),
        );
        small_spheres.push(
            left.clone()
                .transform(
                    translation(i as Real, 0.0, 0.0)
                        * scaling(component_scale, component_scale, component_scale),
                )
                .material(
                    Material::default()
                        .color(colors::new(0.5, 0.6, 1.0))
                        .diffuse(0.7)
                        .specular(0.3)
                        .pattern_opt(if i % 2 == 0 { Some(pattern) } else { None })
                        .reflective(0.5),
                ),
        )
    }

    let group = Rc::new(Shape::empty_group());

    if let Geo::Group(g) = &group.geo {
        g.add_child(Rc::downgrade(&group), Rc::new(left));
        small_spheres
            .into_iter()
            .for_each(|sphere| g.add_child(Rc::downgrade(&group), Rc::new(sphere)));
    }

    group
}

fn cylinders() -> Rc<Shape> {
    let colors = [
        (40.0, 103.0, 160.0),
        (72.0, 120.0, 170.0),
        (99.0, 141.0, 187.0),
        (121.0, 158.0, 196.0),
        (157.0, 179.0, 208.0),
    ];
    let offset_scale = 0.8;
    let left_offset = 2.1;
    let mut cyls = vec![CylLike::cylinder()
        .min(-0.1)
        .max(0.1)
        .to_shape()
        .material(Material::default().color(colors::new(7.0 / 255.0, 87.0 / 255.0, 152.0 / 255.0)))
        .scale(offset_scale, 1.0, offset_scale)
        .translate(left_offset, 0.1, 0.5)];

    let (mut last_min, mut last_max) = (-0.1, 0.1);

    for (i, (r, g, b)) in colors.iter().enumerate() {
        // each cylinder is thinner than the previous one
        let scale_factor = offset_scale - ((i + 1) as f64 * 0.2);

        let scale_factor = if scale_factor >= 0.2 {
            scale_factor
        } else {
            offset_scale / 2_f64.powi(i as i32)
        };

        last_min -= 0.1;
        last_max += 0.1;

        let new_cyl = CylLike::cylinder()
            .min(last_min)
            .max(last_max)
            .to_shape()
            .material(Material::default().color(colors::new(r / 255.0, g / 255.0, b / 255.0)))
            .scale(scale_factor, 1.0, scale_factor)
            .translate(left_offset, last_max, 0.5);

        cyls.push(new_cyl);
    }

    let group = Rc::new(Shape::empty_group());
    if let Geo::Group(g) = &group.geo {
        g.add_children(
            Rc::downgrade(&group),
            cyls.into_iter().map(|cyl| Rc::new(cyl)).collect(),
        );
    }
    group
}

fn glasses() -> Rc<Shape> {
    let upper_base = CylLike::cylinder()
        .closed(true)
        .min(-0.025)
        .max(0.025)
        .to_shape()
        .transform(translation(0.7, 0.575, -1.5) * scaling(0.3, 1.0, 0.3))
        .material(Material::glass());
    let body = CylLike::cylinder()
        .closed(true)
        .min(-0.275)
        .max(0.275)
        .to_shape()
        .transform(translation(0.7, 0.275, -1.5) * scaling(0.05, 1.0, 0.05))
        .material(Material::glass());
    let sphere = Shape::sphere()
        .transform(translation(0.7, 0.85, -1.5) * scaling(0.25, 0.25, 0.25))
        .material(Material::glass());
    let small_sphere = Shape::sphere()
        .transform(translation(0.7, 1.25, -1.5) * scaling(0.15, 0.15, 0.15))
        .material(Material::glass());

    let group = Rc::new(Shape::empty_group());
    if let Geo::Group(g) = &group.geo {
        g.add_child(Rc::downgrade(&group), Rc::new(upper_base));
        g.add_child(Rc::downgrade(&group), Rc::new(body));
        g.add_child(Rc::downgrade(&group), Rc::new(sphere));
        g.add_child(Rc::downgrade(&group), Rc::new(small_sphere));
    }
    group
}

fn cones() -> Vec<Shape> {
    let base_color = colors::new(1.0, 168.0 / 255.0, 18.0 / 255.0);
    let cone = CylLike::cone()
        .min(-1.0)
        .max(0.0)
        .closed(true)
        .to_shape()
        .material(
            Material::default().pattern(
                Pattern::stripe(Color::white(), base_color)
                    .scale(0.15, 0.15, 0.15)
                    .rotate_z(math::PI / 2.0),
            ),
        )
        .scale(0.5, 1.5, 0.5)
        .translate(-3.5, 1.6, 4.5);
    let base = CylLike::cylinder()
        .closed(true)
        .min(-0.1)
        .max(0.1)
        .to_shape()
        .material(Material::default().color(base_color))
        .scale(0.6, 1.0, 0.6)
        .translate(-3.5, 0.1, 4.5);
    vec![base, cone]
}

fn triangles() -> Rc<Shape> {
    let side = Shape::triangle(
        points::new(0.0, 2_f64.sqrt(), 0.0),
        points::new(-1.0, 0.0, 0.0),
        points::new(1.0, 0.0, 0.0),
    )
    .material(Material::glass());

    let side1 = side.clone().rotate_x(math::DEG_45);
    let side2 = side
        .clone()
        .rotate_y(math::DEG_45 * 2.0)
        .rotate_z(-math::DEG_45)
        .translate(-1.0, 0.0, 1.0);
    let side3 = side
        .clone()
        .rotate_x(-math::DEG_45)
        .translate(0.0, 0.0, 2.0);
    let side4 = side
        .clone()
        .rotate_y(-math::DEG_45 * 2.0)
        .rotate_z(math::DEG_45)
        .translate(1.0, 0.0, 1.0);

    let sphere = Shape::sphere()
        .material(
            Material::default()
                .color(colors::new(0.1, 0.1, 0.6))
                .reflective(0.5),
        )
        .scale_all(0.2)
        .translate(-0.5, 0.2, 0.5);

    let cylinder = CylLike::cylinder()
        .min(-0.2)
        .max(0.2)
        .closed(true)
        .to_shape()
        .material(
            Material::default()
                .color(colors::new(0.6, 0.1, 0.1))
                .reflective(0.5),
        )
        .scale(0.2, 1.0, 0.2)
        .translate(0.0, 0.2, 1.0);

    let cube = Shape::cube()
        .material(
            Material::default()
                .color(colors::new(0.1, 0.6, 0.1))
                .reflective(0.5),
        )
        .scale_all(0.17)
        .translate(0.3, 0.17, 0.5);

    let groups = Rc::new(Shape::empty_group().translate(4.5, 0.0, 4.0));
    if let Geo::Group(g) = &groups.geo {
        g.add_children(
            Rc::downgrade(&groups),
            vec![
                Rc::new(side1),
                Rc::new(side2),
                Rc::new(side3),
                Rc::new(side4),
                Rc::new(sphere),
                Rc::new(cylinder),
                Rc::new(cube),
            ],
        );
    }
    groups
}
