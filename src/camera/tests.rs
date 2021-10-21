use crate::math;
use crate::camera::Camera;
use crate::matrix::Matrix;

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