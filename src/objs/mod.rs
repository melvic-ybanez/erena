use std::io::{Lines, BufReader, BufRead, Read, ErrorKind};
use std::fs::File;
use std::io;
use crate::math::Real;
use std::num::ParseFloatError;
use std::error::Error;
use crate::tuples::points::Point;
use crate::tuples::points;

pub struct Parser {
    vertices: Vec<Point>
}

impl Parser {
    pub fn new(mut vertices: Vec<Point>) -> Parser {
        let mut xs = vec![Point::origin()];
        xs.append(&mut vertices);

        Parser { vertices: xs }
    }

    pub fn len(&self) -> usize {
        self.vertices.len() - 1
    }

    pub fn get_vertices(&self) -> Vec<Point> {
        self.vertices.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.len() == 1
    }
}

pub fn parse_obj<R: Read>(read: R) -> Parser {
    let lines = BufReader::new(read).lines();
    let mut result: Vec<Point> = vec![];

    for line in lines {
        if let Ok(line) = line {
            if let Some(point) = parse_line(line) {
                result.push(point);
            }
        }
    }

    Parser::new(result)
}

fn parse_line(line: String) -> Option<Point> {
    let mut line = line.split_whitespace();

    fn parse<F>(r: Option<&str>, f: F) -> Option<Point> where F: FnOnce(Real) -> Option<Point> {
        r.and_then(|r| {
            r.parse::<Real>().ok().and_then(f)
        })
    }

    line.next().and_then(|v| {
        if v != "v" {
            None
        } else {
            parse(line.next(), |x| parse(line.next(), |y| parse(line.next(), |z|
                Some(points::new(x, y, z)))))
        }
    })
}

#[cfg(test)]
mod tests;