use crate::scene::World;
use crate::lights::PointLight;
use crate::tuples::{points, colors};
use crate::tuples::colors::Color;
use crate::shapes::{Sphere, Shape};
use crate::matrix::scaling;

#[test]
fn test_creating_world() {
    let world = World::new();
    assert!(world.is_empty());
}

#[test]
fn test_default_world() {
    let light = PointLight::new(points::new(-10.0, 10.0, -10.0), Color::white());

    let mut s1 = Sphere::new();
    s1.material.color = colors::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let mut s2 = Sphere::new();
    s2.transform(scaling(0.5, 0.5, 0.5));

    let world = World::default();
    assert_eq!(world.light, Some(light));
    assert!(world.contains(Box::new(s1)));
    assert!(world.contains(Box::new(s2)));
}