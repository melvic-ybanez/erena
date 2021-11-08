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

pub struct Parser<'a> {
    vertices: Vec<Point>,
    faces: Vec<&'a [usize; 3]>
}

pub enum LineParseResult {
    Vertex(Point),
    Face(usize, usize, usize),
    None,
}

impl<'a> Parser<'a> {
    pub fn new(mut vertices: Vec<Point>, faces: Vec<&'a [usize; 3]>) -> Parser<'a> {
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

pub fn parse_obj<'a, R: Read>(read: R) -> Parser<'a> {
    let lines = BufReader::new(read).lines();
    let mut result: Vec<Point> = vec![];

    for line in lines {
        if let Ok(line) = line {
            if let LineParseResult::Vertex(point) = parse_line(line) {
                result.push(point);
            }
        }
    }

    Parser::new(result, vec![])
}

fn parse_line(line: String) -> LineParseResult {
    let mut line = line.split_whitespace();

    let first = line.next();
    if let Some("v") = first {
        parse_vertex(line)
    } else if let Some("f") = first {
        parse_face(line)
    } else {
        LineParseResult::None
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
    unimplemented!()
}

#[cfg(test)]
mod tests;