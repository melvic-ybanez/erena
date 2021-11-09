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
use std::rc::Rc;
use crate::shapes::{Shape, Geo};

pub struct Parser {
    vertices: Vec<Point>,
    faces: Vec<FaceData>,
}

pub enum Command {
    Vertex(Point),
    Face(FaceData),
    None,
}

type FaceData = Vec<usize>;

impl Parser {
    pub fn new(mut vertices: Vec<Point>, faces: Vec<FaceData>) -> Parser {
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

    pub fn default_group(&self) -> Rc<Shape> {
        let group = Rc::new(Shape::empty_group());
        for face in self.faces.iter() {
            let triangles = self.fan_triangulation(face);
            for triangle in triangles {
                let triangle = Rc::new(Shape::new(Geo::Triangle(triangle)));
                if let Geo::Group(g) = &group.geo {
                    g.add_child(Rc::downgrade(&group), Rc::clone(&triangle));
                }
            }
        }
        group
    }

    /// Converts polygons into triangles
    fn fan_triangulation(&self, face: &FaceData) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = vec![];
        for i in 1..face.len() - 1 {
            let triangle = Triangle::new(
                self.vertices[face[0]],
                self.vertices[face[i]],
                self.vertices[face[i + 1]]
            );
            triangles.push(triangle)
        }
        triangles
    }
}

pub fn parse_obj<R: Read>(read: R) -> Parser {
    let lines = BufReader::new(read).lines();
    let mut vertices: Vec<Point> = vec![];
    let mut faces: Vec<FaceData> = vec![];

    for line in lines {
        if let Ok(line) = line {
            match parse_line(line) {
                Command::Vertex(point) => vertices.push(point),
                Command::Face(face) => faces.push(face),
                _ => ()
            }
        }
    }

    Parser::new(vertices, faces)
}

fn parse_line(line: String) -> Command {
    let mut line = line.split_whitespace();

    match line.next() {
        Some("v") => parse_vertex(line),
        Some("f") => parse_face(line),
        _ => Command::None
    }
}

fn parse_vertex(mut line: SplitWhitespace) -> Command {
    let mut components: Vec<Real> = vec![];

    for word in line {
        if let Ok(component) = word.parse::<Real>() {
            components.push(component);
        }
    }

    if components.is_empty() {
        Command::None
    } else {
        Command::Vertex(points::new(components[0], components[1], components[2]))
    }
}

fn parse_face(mut line: SplitWhitespace) -> Command {
    let mut vertex_indices: Vec<usize> = vec![];

    for word in line {
        if let Ok(vertex_index) = word.parse::<usize>() {
            vertex_indices.push(vertex_index);
        }
    }

    if vertex_indices.is_empty() {
        Command::None
    } else {
        Command::Face(vertex_indices)
    }
}

#[cfg(test)]
mod tests;