use crate::math::Real;
use std::ops::Index;
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

impl Index<usize> for Canvas {
    type Output = [Color];

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index * self.width..(index + 1) * self.width]
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
}