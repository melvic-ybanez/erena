use crate::matrix::Matrix;

pub struct Camera {
    pub width: i32,
    pub height: i32,
    pub field_of_view: f64,
    pub transformation: Matrix,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    fn new(width: i32, height: i32, field_of_view: f64) -> Camera {
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
}

#[cfg(test)]
mod tests;