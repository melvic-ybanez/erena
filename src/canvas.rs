use crate::math::Real;
use std::ops::{Index, IndexMut};
use crate::tuples::Color;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        let pixels = (0..(width * height)).map(|x| Color::BLACK).collect();
        Canvas { width, height, pixels }
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.pixels[row *  self.width + col]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        &mut self.pixels[row * self.width + col]
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::tuples::Color;

    #[test]
    fn test_canvas_creation() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        canvas.pixels.iter().map(|color| assert_eq!(*color, Color::BLACK));
    }

    #[test]
    fn test_pixel_update() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::RED;
        canvas[(2, 3)] = red;
        assert_eq!(canvas[(2, 3)], red);
    }
}