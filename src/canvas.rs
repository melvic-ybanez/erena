use crate::math::Real;
use std::ops::{Index, IndexMut};
use crate::tuples::{Color, Tuple};
use crate::math;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        let pixels = (0..(width * height)).map(|x| Color::BLACK).collect();
        Canvas { width, height, pixels }
    }

    fn to_ppm(&self) -> Ppm {
        let header = format!("P3\n{} {}\n{}", self.width, self.height, 255);

        fn row(chunk: &[Color]) -> String {
            let row: Vec<_> = chunk.iter().map(|color| {
                let Tuple { x: r, y: g, z: b, .. } = color.0;
                let max_value = Ppm::MAX_COLOR_VALUE as i32;
                let r = math::scale_to(max_value, r);
                let g = math::scale_to(max_value, g);
                let b = math::scale_to(max_value, b);
                format!("{} {} {}", r, g, b)
            }).collect();
            row.join(" ")
        }

        let data: Vec<_> = self.pixels.chunks(self.width).map(row).collect();
        Ppm::new(header, data)
    }
}

struct Ppm {
    header: String,
    data: Vec<String>,
}

impl Ppm {
    const MAX_COLOR_VALUE: u8 = 255;

    fn new(header: String, data: Vec<String>) -> Ppm {
        Ppm { header, data }
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.pixels[math::index_of(x, y, self.width)]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.pixels[math::index_of(x, y, self.width)]
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

    #[test]
    fn test_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();
        assert_eq!(ppm.header, "P3\n5 3\n255");
    }

    #[test]
    fn test_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        canvas[(0, 0)] = c1;
        canvas[(2, 1)] = c2;
        canvas[(4, 2)] = c3;
        let ppm = canvas.to_ppm();
        assert_eq!(ppm.data, vec!(
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
        ));
    }
}