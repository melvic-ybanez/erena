use crate::materials::Material;
use crate::tuples::colors::Color;

#[test]
fn test_default_material() {
    let mat = Material::default();
    assert_eq!(mat.color, Color::white());
    assert_eq!(mat.ambient, 0.1);
    assert_eq!(mat.diffuse, 0.9);
    assert_eq!(mat.specular, 0.9);
    assert_eq!(mat.shininess, 200.0);
}