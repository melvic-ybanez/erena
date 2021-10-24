use crate::canvas::Canvas;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::scene::World3D;
use crate::tuples::points;
use crate::tuples::points::Point;

pub struct Camera {
    pub width: usize,
    pub height: usize,
    pub field_of_view: f64,
    pub transformation: Matrix,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub(crate) fn new(width: usize, height: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = width as f64 / height as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = half_width * 2.0 / (width as f64);

        Camera {
            width,
            height,
            field_of_view,
            transformation: Matrix::id44(),
            pixel_size,
            half_width,
            half_height
        }
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // compute the offsets from
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        // compute the untransformed coordinates of the pixel in the world space
        // note: the camera looks toward -z, so +x is to the left
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let inverse = self.transformation.inverse_or_id44();
        let pixel = &inverse * points::new(world_x, world_y, -1.0);
        let origin = inverse * Point::origin();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub(crate) fn render(&self, world: World3D) -> Canvas {
        let mut image = Canvas::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let ray = self.ray_for_pixel(x, y);
                image[(x, y)] = world.color_at(&ray);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests;