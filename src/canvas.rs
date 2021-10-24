use std::ops::{Index, IndexMut};

use crate::math;
use crate::tuples::colors::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas { width, height, pixels: vec![Color::black(); width * height] }
    }

    pub fn to_ppm(&self) -> Ppm {
        let header = format!("P3\n{} {}\n{}", self.width, self.height, Ppm::MAX_COLOR_VALUE);

        fn row(chunk: &[Color]) -> Vec<String> {
            let row: Vec<_> = chunk.iter().map(|color| {
                let Color { x: r, y: g, z: b, .. } = color;
                let max_value = Ppm::MAX_COLOR_VALUE as i32;
                let r = math::scale_to(max_value, *r);
                let g = math::scale_to(max_value, *g);
                let b = math::scale_to(max_value, *b);
                format!("{} {} {}", r, g, b)
            }).collect();
            Canvas::wrap(row.join(" "))
        }

        let data: Vec<_> = self.pixels.chunks(self.width).flat_map(row).collect();
        Ppm::new(header, data)
    }

    fn wrap(row: String) -> Vec<String> {
        row.split(" ").fold(vec![], |mut acc, next| {
            let next_str = next.to_string();
            match acc.last_mut() {
                None => vec![next_str],
                Some(last) =>
                    if last.len() + next.len() > 69 {
                        acc.push(next_str);
                        acc
                    } else {
                        *last = last.to_string() + " " + &next;
                        acc
                    }
            }
        })
    }
}

pub struct Ppm {
    header: String,
    data: Vec<String>,
}

impl Ppm {
    const MAX_COLOR_VALUE: u8 = 255;

    fn new(header: String, data: Vec<String>) -> Ppm {
        Ppm { header, data }
    }
}

impl Index<math::Idx> for Canvas {
    type Output = Color;

    fn index(&self, index: math::Idx) -> &Self::Output {
        let (x, y) = index;
        &self.pixels[math::index_of(x, y, self.width)]
    }
}

impl IndexMut<math::Idx> for Canvas {
    fn index_mut(&mut self, index: math::Idx) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.pixels[math::index_of(x, y, self.width)]
    }
}

impl ToString for Ppm {
    fn to_string(&self) -> String {
        format!("{}\n{}\n", self.header, self.data.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::tuples::colors;
    use crate::tuples::colors::Color;

    #[test]
    fn test_canvas_creation() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        canvas.pixels.iter().for_each(|color| assert_eq!(*color, Color::black()));
    }

    #[test]
    fn test_pixel_update() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::red();
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
        let c1 = colors::new(1.5, 0.0, 0.0);
        let c2 = colors::new(0.0, 0.5, 0.0);
        let c3 = colors::new(-0.5, 0.0, 1.0);
        canvas[(0, 0)] = c1;
        canvas[(2, 1)] = c2;
        canvas[(4, 2)] = c3;
        let ppm = canvas.to_ppm();
        assert_eq!(ppm.data, vec![
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
        ]);
    }

    #[test]
    fn test_long_lines_wrapping() {
        let mut canvas = Canvas::new(10, 2);
        for i in 0..10 {
            for j in 0..2 {
                canvas[(i, j)] = colors::new(1.0, 0.8, 0.6);
            }
        }
        let ppm = canvas.to_ppm();
        assert_eq!(ppm.data, vec![
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        ])
    }
}