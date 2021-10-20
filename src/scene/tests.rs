use crate::scene::{World, World3D};
use crate::lights::PointLight;
use crate::tuples::{points, colors, vectors};
use crate::tuples::colors::Color;
use crate::shapes::Shape;
use crate::matrix::scaling;
use crate::rays::{Ray, Intersection, Comps3D};
use crate::tuples::points::Point;

#[test]
fn test_creating_world() {
    let world = World3D::new();
    assert!(world.is_empty());
}

#[test]
fn test_default_world() {
    let light = PointLight::new(points::new(-10.0, 10.0, -10.0), Color::white());

    let mut s1 = Shape::sphere();
    s1.material.color = colors::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let mut s2 = Shape::sphere();
    s2.transform(scaling(0.5, 0.5, 0.5));

    let world = World::default();
    assert_eq!(world.light, Some(light));
    assert!(world.contains(s1));
    assert!(world.contains(s2));
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
    let comps = Comps3D::prepare(i, &ray);
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
    let comps = Comps3D::prepare(i, &ray);
    let color = world.shade_hit(comps);
    assert_eq!(color.round_items(), colors::new(0.90498, 0.90498, 0.90498));
}