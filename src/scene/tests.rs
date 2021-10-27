use crate::matrix::{CanTransform, scaling};
use crate::rays::{Comps3D, Intersection, Ray, Comps};
use crate::scene::{World, World3D};
use crate::shapes::Shape;
use crate::tuples::{colors, points, vectors};
use crate::tuples::colors::Color;
use crate::tuples::points::Point;
use crate::materials::Material;
use crate::math;
use crate::rays::lights::PointLight;

#[test]
fn test_creating_world() {
    let world = World3D::empty();
    assert!(world.is_empty());
}

#[test]
fn test_default_world() {
    let light = PointLight::new(points::new(-10.0, 10.0, -10.0), Color::white());

    let mut s1 = Shape::sphere();
    s1.material.color = colors::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let s2 = Shape::sphere().transform(scaling(0.5, 0.5, 0.5));

    let world = World::default();
    assert_eq!(world.light, Some(light));
    assert!(world.contains(&s1));
    assert!(world.contains(&s2));
}

/// Tests intersect a world with ray
#[test]
fn test_intersect() {
    let world = World::default();
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let xs = world.intersect(&ray);
    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 4.5);
    assert_eq!(xs[2].t, 5.5);
    assert_eq!(xs[3].t, 6.0);
}

#[test]
fn test_shading_an_intersection() {
    let world = World::default();
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 0.0, 1.0));
    let shape = &world.objects[0];
    let i = Intersection::new(4.0, shape);
    let comps = Comps3D::prepare_default(i, &ray);
    let color = world.shade_hit(comps);
    assert_eq!(color.round_items(), colors::new(0.38066, 0.47583, 0.28550));
}

/// Tests shading an intersection from the inside
#[test]
fn test_shading_from_inside() {
    let mut world = World::default();
    world.light = Some(PointLight::new(points::new(0.0, 0.25, 0.0), Color::white()));
    let ray = Ray::new(Point::origin(), vectors::new(0.0, 0.0, 1.0));
    let shape = &world.objects[1];
    let i = Intersection::new(0.5, shape);
    let comps = Comps3D::prepare_default(i, &ray);
    let color = world.shade_hit(comps);
    assert_eq!(color.round_items(), colors::new(0.90498, 0.90498, 0.90498));
}

#[test]
fn test_missed_ray_color() {
    let world = World::default();
    let ray = Ray::new(points::new(0.0, 0.0, -5.0), vectors::new(0.0, 1.0, 0.0));
    let color = world.color_at(&ray);
    assert_eq!(color, Color::black());
}

/// Tests the color with an intersection behind the ray
#[test]
fn test_behind_ray_color() {
    let mut world = World::default();

    let (ray, inner_color) = {
        let outer = &mut world.objects[0];
        outer.material.ambient = 1.0;
        let inner = &mut world.objects[1];
        inner.material.ambient = 1.0;

        // ray is inside the outer sphere, but outside the inner sphere
        let ray = Ray::new(points::new(0.0, 0.0, 0.75), vectors::new(0.0, 0.0, -1.0));
        (ray, inner.material.color)
    };

    let color = world.color_at(&ray);
    assert_eq!(color, inner_color);
}

/// Tests that there is no shadow when nothing is collinear with the point
/// and light. It means nothing lies along the vector connecting the light
/// source and the object.
#[test]
fn test_no_collinear() {
    let world = World::default();
    let point = points::new(0.0, 10.0, 0.0);
    assert!(!world.is_shadowed(point));
}

/// Tests the shadow when an object is between the point and the light
#[test]
fn test_object_shadow_between_point_and_light() {
    let world = World::default();
    let point = points::new(10.0, -10.0, 10.0);
    assert!(world.is_shadowed(point));
}

/// Tests that there is no shadow when an object is behind the light
#[test]
fn test_no_object_shadow_behind_light() {
    let world = World::default();
    let point = points::new(-20.0, 20.0, -20.0);
    assert!(!world.is_shadowed(point));
}

/// Tests that there is no shadow when an object is behind the point
#[test]
fn test_no_object_shadow_behind_point() {
    let world = World::default();
    let point = points::new(-2.0, 2.0, -2.0);
    assert!(!world.is_shadowed(point));
}

/// Tests that shade-hit is given an intersection in shadow
#[test]
fn test_shade_hit_intersection_in_shadow() {
    let mut world = World::empty();
    world.add_light(PointLight::new(points::new(0.0, 0.0, -10.0), Color::white()));

    let sphere1 = Shape::sphere();
    world.add_object(&sphere1);

    let sphere2 = Shape::sphere().translate(0.0, 0.0, 10.0);
    world.add_object(&sphere2);

    let ray = Ray::new(points::new(0.0, 0.0, 5.0), vectors::new(0.0, 0.0, 1.0));
    let intersection = Intersection::new(4.0, &sphere2);

    let comps = Comps::prepare_default(intersection, &ray);
    let result = world.shade_hit(comps);

    assert_eq!(result, colors::new(0.1, 0.1, 0.1));
}

/// The reflected color for a non-reflective material
#[test]
fn test_non_reflective_mat_reflection() {
    let mut world = World::default();
    let ray = Ray::new(Point::origin(), vectors::new(0.0, 0.0, 1.0));

    world.update_object(1, |mut shape| {
        shape.material.ambient = 1.0;
        shape
    });
    let shape = &world.objects[1];
    let i = Intersection::new(1.0, shape);

    let comps = Comps::prepare_default(i, &ray);
    let color = world.reflected_color(comps);
    assert_eq!(color, Color::black());
}

/// The reflected color for a reflective material
#[test]
fn test_reflective_mat_reflection() {
    let mut world = World::default();
    let shape = Shape::plane().material(
        Material::default().reflective(0.5)
    ).translate(0.0, -1.0, 0.0);
    world.add_object(&shape);
    let ray = Ray::new(
        points::new(0.0, 0.0, -3.0),
        vectors::new(0.0, -math::two_sqrt_div_2(), math::two_sqrt_div_2()),
    );
    let i = Intersection::new(2_f64.sqrt(), &shape);
    let comps = Comps::prepare_default(i, &ray);
    let color = world.reflected_color(comps);
    assert_eq!(color.round_items(), colors::new(0.19033, 0.23791, 0.14275));
}

/// Shade-hit with a reflective material
#[test]
fn test_shade_with_reflective_mat() {
    let mut world = World::default();
    let shape = Shape::plane()
        .material(Material::default().reflective(0.5))
        .translate(0.0, -1.0, 0.0);
    world.add_object(&shape);
    let ray = Ray::new(
        points::new(0.0, 0.0, -3.0),
        vectors::new(0.0, -math::two_sqrt_div_2(), math::two_sqrt_div_2()),
    );
    let i = Intersection::new(2_f64.sqrt(), &shape);
    let comps = Comps::prepare_default(i, &ray);
    let color = world.shade_hit(comps);
    assert_eq!(color.round_items(), colors::new(0.87676, 0.92434, 0.82917));
}

/// Color-at with mutually reflective surfaces
#[test]
fn test_mutually_reflective_surfaces_color() {
    let mut world = World::default();
    world.add_light(PointLight::new(Point::origin(), Color::white()));

    let lower = Shape::plane()
        .material(Material::default().reflective(1.0))
        .translate(0.0, -1.0, 0.0);
    world.add_object(&lower);

    let upper = Shape::plane()
        .material(Material::default().reflective(1.0))
        .translate(0.0, 1.0, 0.0);
    world.add_object(&upper);

    let ray = Ray::new(Point::origin(), vectors::new(0.0, 1.0, 0.0));
    world.color_at(&ray); // should terminate successfully
    assert!(true)
}

#[test]
fn test_reflected_color_at_max_recursive_depth() {
    let mut world = World::default();
    let shape = Shape::plane()
        .material(Material::default().reflective(0.5))
        .translate(0.0, -1.0, 0.0);
    world.add_object(&shape);
    let ray = Ray::new(
        points::new(0.0, 0.0, -3.0),
        vectors::new(0.0, -math::two_sqrt_div_2(), math::two_sqrt_div_2())
    );
    let i = Intersection::new(2_f64.sqrt(), &shape);
    let comps = Comps::prepare_default(i, &ray);
    let color = world.reflected_color_with_depth(comps, 0);
    assert_eq!(color, Color::black());
}