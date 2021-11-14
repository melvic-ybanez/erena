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
        assert_eq!(world.is_shadowed_with_light(light_position, point), result);
    }
}