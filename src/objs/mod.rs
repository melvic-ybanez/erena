use std::io::{Lines, BufReader, BufRead, Read, ErrorKind};
use std::fs::File;
use std::io;
use crate::math::Real;
use std::num::ParseFloatError;
use std::error::Error;
use crate::tuples::points::{Point, PointT};
use crate::tuples::{points, TupleLike};
use crate::shapes::groups::Group;
use std::str::SplitWhitespace;
use crate::shapes::triangles::Triangle;

pub struct Parser {
    vertices: Vec<Point>,
    faces: Vec<[usize; 3]>,
}

pub enum LineParseResult {
    Vertex(Point),
    Face(usize, usize, usize),
    None,
}

impl Parser {
    pub fn new(mut vertices: Vec<Point>, faces: Vec<[usize; 3]>) -> Parser {
        // this is a quick way to make the indices 1-based
        let mut xs = vec![Point::origin()];
        xs.append(&mut vertices);

        Parser { vertices: xs, faces }
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

    pub fn default_group(&self) -> Group {
        unimplemented!()
    }
}

pub fn parse_obj<R: Read>(read: R) -> Parser {
    let lines = BufReader::new(read).lines();
    let mut vertices: Vec<Point> = vec![];
    let mut faces: Vec<[usize; 3]> = vec![];

    for line in lines {
        if let Ok(line) = line {
            match parse_line(line) {
                LineParseResult::Vertex(point) => vertices.push(point),
                LineParseResult::Face(v1, v2, v3) => faces.push([v1, v2, v3]),
                _ => ()
            }
        }
    }

    Parser::new(vertices, faces)
}

fn parse_line(line: String) -> LineParseResult {
    let mut line = line.split_whitespace();

    match line.next() {
        Some("v") => parse_vertex(line),
        Some("f") => parse_face(line),
        _ => LineParseResult::None
    }
}

fn parse_vertex(mut line: SplitWhitespace) -> LineParseResult {
    fn parse<F>(r: Option<&str>, f: F) -> Option<Point> where F: FnOnce(Real) -> Option<Point> {
        r.and_then(|r| {
            r.parse::<Real>().ok().and_then(f)
        })
    }

    let result = parse(line.next(), |x| parse(line.next(), |y| parse(line.next(), |z|
        Some(points::new(x, y, z)))));
    match result {
        None => LineParseResult::None,
        Some(point) => LineParseResult::Vertex(point)
    }
}

fn parse_face(mut line: SplitWhitespace) -> LineParseResult {
    type FaceData = (usize, usize, usize);

    fn parse<F>(r: Option<&str>, f: F) -> Option<FaceData>
        where F: FnOnce(usize) -> Option<FaceData> {
        r.and_then(|r| {
            r.parse::<usize>().ok().and_then(f)
        })
    }

    let result = parse(line.next(), |v1| parse(line.next(), |v2| parse(line.next(), |v3|
        Some((v1, v2, v3)))));
    match result {
        None => LineParseResult::None,
        Some((v1, v2, v3)) => LineParseResult::Face(v1, v2, v3)
    }
}

#[cfg(test)]
mod tests;