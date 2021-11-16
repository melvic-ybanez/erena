use crate::scene::World;
use crate::tuples::points;

#[test]
fn test_occlusion_between_two_points() {
    let world = World::default();
    let light_position = points::new(-10.0, -10.0, -10.0);
    let data = [
        (points::new(-10.0, -10.0, 10.0), false),
        (points::new(10.0, 10.0, 10.0), true),
        (points::new(-20.0, -20.0, -20.0), false),
        (points::new(-20.0, -20.0, -20.0), false),
        (points::new(-5.0, -5.0, -5.0), false)
    ];
    for (point, result) in data {
        assert_eq!(world.is_shadowed(light_position, point), result);
    }
}

/// Tests if point lights evaluate the light intensity at
/// a given point
#[test]
fn test_intensity_for_point_light() {
    let world = World::default();
    let light = world.get_light().expect("Default world requires a light");
    let data = [
        (0.0, 1.0001, 0.0, 1.0),
        (-1.0001, 0.0, 1.0, 1.0),
        (0.0, 0.0, -1.0001, 1.0),
        (0.0, 0.0, 1.0001, 0.0),
        (1.0001, 0.0, 0.0, 0.0),
        (0.0, -1.0001, 0.0, 0.0),
        (0.0, 0.0, 0.0, 0.0)
    ];
    for (x, y, z, result) in data {
        let point = points::new(x, y, z);
        assert_eq!(light.intensity_at(point, &world), result);
    }
}