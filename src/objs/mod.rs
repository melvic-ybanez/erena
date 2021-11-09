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
use crate::shapes::{Shape, Geo, Object};
use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::Borrow;

pub struct Parser<'a> {
    vertices: Vec<Point>,
    faces: Vec<FaceData>,
    groups: HashMap<&'a str, Rc<Shape>>,
}

pub enum Statement<'a> {
    Vertex(Point),
    Face(FaceData),
    Group(&'a str),
    None,
}

type FaceData = Vec<usize>;

impl<'a> Parser<'a> {
    pub fn new(mut vertices: Vec<Point>, faces: Vec<FaceData>) -> Parser<'a> {
        // this is a quick way to make the indices 1-based
        let mut xs = vec![Point::origin()];
        xs.append(&mut vertices);

        Parser { vertices: xs, faces, groups: HashMap::new() }
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

    pub fn default_group(&mut self) -> Rc<Shape> {
        let key = "default";
        for face in self.faces.iter() {
            self.register_face(key, face);
        }
        self.groups.get(key).map(|r| *r).unwrap_or(Rc::new(Shape::empty_group()))
    }

    fn register_face(&mut self, name: &'a str, face: &FaceData) -> Option<Rc<Shape>> {
        let group = match self.groups.get_mut(name) {
            None => Rc::new(Shape::empty_group()),
            Some(group) => group.clone()
        };

        let triangles = self.fan_triangulation(face);
        for triangle in triangles {
            let triangle = Rc::new(Shape::new(Geo::Triangle(triangle)));
            if let Geo::Group(g) = &group.geo {
                g.add_child(Rc::downgrade(&group), Rc::clone(&triangle));
            }
        }

        self.groups.insert(name, group)
    }

    /// Converts polygons into triangles
    fn fan_triangulation(&self, face: &FaceData) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = vec![];
        for i in 1..face.len() - 1 {
            let triangle = Triangle::new(
                self.vertices[face[0]],
                self.vertices[face[i]],
                self.vertices[face[i + 1]],
            );
            triangles.push(triangle)
        }
        triangles
    }

    pub fn get_group(&self, name: &str) -> Group {
        unimplemented!()
    }

    pub fn get_triangle(&'a self, name: &str, i: usize) -> Option<Triangle> {
        let triangle_shape = self.get_group(name).get_child(i);
        if let Geo::Triangle(triangle) = triangle_shape.geo {
            Some(triangle)
        } else {
            None
        }
    }

    pub fn get_triangle_unsafe(&'a self, name: &'a str, i: usize) -> Triangle {
        self.get_triangle(name, i).unwrap()
    }
}

pub fn parse_obj<'a, R: Read>(read: R) -> Parser<'a> {
    let lines = BufReader::new(read).lines();
    let mut current_group: Option<&str> = None;
    let mut parser = Parser::new(vec![], vec![]);

    for line in lines {
        if let Ok(line) = line {
            match parse_statement(line) {
                Statement::Vertex(point) => parser.vertices.push(point),
                Statement::Face(face) => {
                    if let Some(group) = current_group {
                        parser.register_face(group, &face);
                    }
                    parser.faces.push(face)
                },
                Statement::Group(name) => current_group = Some(name),
                _ => ()
            }
        }
    }
    parser
}

fn parse_statement<'a>(line: String) -> Statement<'a> {
    let mut line = line.split_whitespace();

    match line.next() {
        Some("v") => parse_vertex(line),
        Some("f") => parse_face(line),
        Some("g") => parse_group(line),
        _ => Statement::None
    }
}

fn parse_vertex(line: SplitWhitespace) -> Statement {
    let mut components: Vec<Real> = vec![];

    for word in line {
        if let Ok(component) = word.parse::<Real>() {
            components.push(component);
        }
    }

    if components.is_empty() {
        Statement::None
    } else {
        Statement::Vertex(points::new(components[0], components[1], components[2]))
    }
}

fn parse_face(line: SplitWhitespace) -> Statement {
    let mut vertex_indices: Vec<usize> = vec![];

    for word in line {
        if let Ok(vertex_index) = word.parse::<usize>() {
            vertex_indices.push(vertex_index);
        }
    }

    if vertex_indices.is_empty() {
        Statement::None
    } else {
        Statement::Face(vertex_indices)
    }
}

fn parse_group(mut line: SplitWhitespace) -> Statement {
    match line.next() {
        None => Statement::None,
        Some(name) => Statement::Group(name)
    }
}

#[cfg(test)]
mod tests;