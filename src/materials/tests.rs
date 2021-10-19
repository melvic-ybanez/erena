use crate::materials::Material;
use crate::tuples::colors::Color;
use crate::tuples::points::Point;
use crate::tuples::{vectors, points, colors};
use crate::lights::PointLight;

fn set_up() -> (Material, Point) {
    (Material::default(), Point::origin())
}

#[test]
fn test_default_material() {
    let mat = Material::default();
    assert_eq!(mat.color, Color::white());
    assert_eq!(mat.ambient, 0.1);
    assert_eq!(mat.diffuse, 0.9);
    assert_eq!(mat.specular, 0.9);
    assert_eq!(mat.shininess, 200.0);
}

/// Tests lighting with the eye between the light and the surface
#[test]
fn test_lighting_in_between() {
    let (mat, position) = set_up();
    let eye_vec = vectors::new(0.0, 0.0, -1.0);
    let normal_vec = vectors::new(0.0, 0.0, -1.0);
    let light = PointLight::new(points::new(0.0, 0.0, -10.0), Color::white());
    let result = mat.lighting(light, position, eye_vec, normal_vec);
    assert_eq!(result, colors::new(1.9, 1.9, 1.9));
}


/// Tests lighting with the eye in between light and surface, eye offset 45 degrees
#[test]
fn test_lighting_in_between_offset_45() {
    let (mat, position) = set_up();
    let eye_vec = vectors::new(0.0, 2_f64.sqrt() / 2.0,  -2_f64.sqrt() / 2.0);
    let normal_vec = vectors::new(0.0, 0.0, -1.0);
    let light = PointLight::new(points::new(0.0, 0.0, -10.0), Color::white());
    let result = mat.lighting(light, position, eye_vec, normal_vec);
    assert_eq!(result, Color::white());
}


/// Tests lighting with the eye opposite surface, light offset 45%
#[test]
fn test_lighting_opposite_surface_offset_45() {
    let (mat, position) = set_up();
    let eye_vec = vectors::new(0.0, 0.0, -1.0);
    let normal_vec = vectors::new(0.0, 0.0, -1.0);
    let light = PointLight::new(points::new(0.0, 10.0, -10.0), Color::white());
    let result = mat.lighting(light, position, eye_vec, normal_vec);
    assert_eq!(result.round_items(), colors::new(0.7364, 0.7364, 0.7364));
}

/// Tests lighting with eye in the path of the reflection vector
#[test]
fn test_in_reflection_path() {
    let (mat, position) = set_up();
    let eye_vec = vectors::new(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
    let normal_vec = vectors::new(0.0, 0.0, -1.0);
    let light = PointLight::new(points::new(0.0, 10.0, -10.0), Color::white());
    let result = mat.lighting(light, position, eye_vec, normal_vec);
    assert_eq!(result.round_items(), colors::new(1.6364, 1.6364, 1.6364));
}

/// Tests lighting with the light behind the surface
#[test]
fn test_lighting_behind_the_surface() {
    let (mat, position) = set_up();
    let eye_vec = vectors::new(0.0, 0.0, -1.0);
    let normal_vec = vectors::new(0.0, 0.0, -1.0);
    let light = PointLight::new(points::new(0.0, 0.0, 10.0), Color::white());
    let result = mat.lighting(light, position, eye_vec, normal_vec);
    assert_eq!(result, colors::new(0.1, 0.1, 0.1));
}