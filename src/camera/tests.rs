use crate::math;
use crate::camera::Camera;
use crate::matrix::{Matrix, rotation_y, translation, view_transformation};
use crate::tuples::points::Point;
use crate::tuples::{vectors, points, colors};
use crate::scene::World;

#[test]
fn test_constructing_camera() {
    let width = 160;
    let height = 120;
    let field_of_view = math::PI / 2.0;
    let camera = Camera::new(width, height, field_of_view);

    assert_eq!(camera.width, width);
    assert_eq!(camera.height, height);
    assert_eq!(camera.field_of_view, field_of_view);
    assert_eq!(camera.transformation, Matrix::id44());
}

/// Tests the pixel size for a horizontal canvas (i.e. a canvas
/// with a horizontal aspect, or width > height).
#[test]
fn test_horizontal_pixel_size() {
    let camera = Camera::new(200, 125, math::PI / 2.0);
    assert_eq!(math::round(camera.pixel_size, 2), 0.01);
}

/// Tests the pixel size for a vertical canvas
#[test]
fn test_vertical_pixel_size() {
    let camera = Camera::new(125, 200, math::PI / 2.0);
    assert_eq!(math::round(camera.pixel_size, 2), 0.01);
}

/// Tests constructing a ray through the center of the canvas
#[test]
fn test_ray_through_center() {
    let camera = Camera::new(201, 101, math::PI / 2.0);
    let ray = camera.ray_for_pixel(100, 50);
    assert_eq!(ray.origin, Point::origin());
    assert_eq!(ray.direction, vectors::new(0.0, 0.0, -1.0));
}

/// Tests constructing a ray through a corner of the canvas
#[test]
fn test_ray_through_corner() {
    let camera = Camera::new(201, 101, math::PI / 2.0);
    let ray = camera.ray_for_pixel(0, 0);
    assert_eq!(ray.origin, Point::origin());
    assert_eq!(ray.direction.round_items(), vectors::new(0.66519, 0.33259, -0.66851));
}

/// Tests constructing a ray when the camera is transformed
#[test]
fn test_ray_with_transformed_camera() {
    let mut camera = Camera::new(201, 101, math::PI / 2.0);
    camera.transformation = rotation_y(math::PI / 4.0) * translation(0.0, -2.0, 5.0);
    let ray = camera.ray_for_pixel(100, 50);

    assert_eq!(ray.origin, points::new(0.0, 2.0, -5.0));
    assert_eq!(ray.direction, vectors::new(2_f64.sqrt() / 2.0, 0.0, -2_f64.sqrt() / 2.0));
}

/// Tests rendering a world with a camera
#[test]
fn test_render_camera() {
    let world = World::default();
    let mut camera = Camera::new(11, 11, math::PI / 2.0);
    let from = points::new(0.0, 0.0, -5.0);
    let to = Point::origin();
    let up = vectors::new(0.0, 1.0, 0.0);
    camera.transformation = view_transformation(from, to, up);
    let image = camera.render(world);
    assert_eq!(image[(5, 5)].round_items(), colors::new(0.38066, 0.47583, 0.28550));
}