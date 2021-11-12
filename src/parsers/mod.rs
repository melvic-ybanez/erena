use std::io::{BufReader, BufRead, Read};


use crate::math::Real;


use crate::tuples::points::{Point};
use crate::tuples::{points, vectors};
use crate::shapes::groups::Group;
use std::str::SplitWhitespace;
use crate::shapes::triangles::Triangle;
use std::rc::Rc;
use crate::shapes::{Shape, Geo};
use std::collections::HashMap;
use crate::tuples::vectors::Vector;

/// Contains information about an input OBJ, which can be represented
/// as a file, a byte slice or any data that implements Read.
/// Note: vertices and normals are both 1-based
pub struct Parser {
    vertices: Vec<Point>,
    normals: Vec<Vector>,
    faces: Vec<FaceData>,
    groups: HashMap<String, Rc<Shape>>,
}

pub enum Statement {
    Vertex(Point),
    Normal(Vector),
    Face(FaceData),
    Group(String),
    None,
}

#[derive(Debug, Clone)]
pub struct FaceData {
    vs: Vec<usize>,
    ns: Vec<usize>,
}

impl FaceData {
    fn new(vs: Vec<usize>, ns: Vec<usize>) -> FaceData {
        FaceData { vs, ns }
    }
}

pub const DEFAULT_GROUP: &str = "default";

impl Parser {
    pub fn new() -> Parser {
        // this is a quick way to make the indices 1-based
        let vertices = vec![Point::origin()];
        let normals = vec![Vector::zero()];

        Parser { vertices, normals, faces: vec![], groups: HashMap::new() }
    }

    pub fn len(&self) -> usize {
        self.vertices.len() - 1
    }

    pub fn get_vertices(&self) -> Vec<Point> {
        self.vertices.clone()
    }

    pub fn get_normals(&self) -> Vec<Vector> {
        self.normals.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.len() == 1
    }

    pub fn get_default_group(&mut self) -> Rc<Shape> {
        self.group_default();
        self.groups.get(DEFAULT_GROUP).map(|r| (*r).clone()).unwrap_or(Rc::new(Shape::empty_group()))
    }

    pub fn group_default(&mut self) {
        self.group_faces(DEFAULT_GROUP, self.faces.clone());
    }

    fn group_faces(&mut self, name: &str, faces: Vec<FaceData>) {
        faces.iter().for_each(|face| {
            self.add_face_to_group(name, face);
        })
    }

    fn add_face_to_group(&mut self, name: &str, data: &FaceData) -> Option<Rc<Shape>> {
        let group = match self.groups.get_mut(name) {
            None => Rc::new(Shape::empty_group()),
            Some(group) => group.clone()
        };

        let triangles = self.fan_triangulation(data);
        for triangle in triangles {
            let triangle = Rc::new(Shape::new(Geo::Triangle(triangle)));
            if let Geo::Group(g) = &group.geo {
                g.add_child(Rc::downgrade(&group), Rc::clone(&triangle));
            }
        }

        self.groups.insert(name.to_string(), group)
    }

    /// Converts polygons into triangles
    fn fan_triangulation(&self, data: &FaceData) -> Vec<Triangle> {
        let FaceData { vs, ns } = data;
        let mut triangles: Vec<Triangle> = vec![];
        for i in 1..vs.len() - 1 {
            let (v1, v2, v3) = (
                self.vertices[vs[0]],
                self.vertices[vs[i]],
                self.vertices[vs[i + 1]],
            );
            let triangle = if ns.is_empty() {
                Triangle::regular(v1, v2, v3)
            } else {
                Triangle::smooth(
                    v1, v2, v3,
                    self.normals[ns[0]],
                    self.normals[ns[i]],
                    self.normals[ns[i + 1]],
                )
            };
            triangles.push(triangle)
        }
        triangles
    }

    pub fn get_group(&self, name: &str) -> Option<&Rc<Shape>> {
        self.groups.get(name)
    }

    pub fn get_group_geo(&self, name: &str) -> Option<Group> {
        self.get_group(name).and_then(|group| {
            if let Geo::Group(g) = &group.geo {
                Some(g.clone())
            } else {
                None
            }
        })
    }

    pub fn get_triangle(&self, name: &str, i: usize) -> Option<Triangle> {
        self.get_group_geo(name)
            .and_then(|group| {
                let child = group.get_child(i);
                if let Geo::Triangle(triangle) = child.geo {
                    Some(triangle.clone())
                } else {
                    None
                }
            })
    }

    pub fn get_triangle_unsafe(&self, name: &str, i: usize) -> Triangle {
        self.get_triangle(name, i).unwrap()
    }
}

pub fn parse_obj<R: Read>(read: R) -> Parser {
    let lines = BufReader::new(read).lines();
    let mut current_group: Option<String> = None;
    let mut parser = Parser::new();

    for line in lines {
        if let Ok(line) = line {
            match parse_statement(line) {
                Statement::Vertex(point) => parser.vertices.push(point),
                Statement::Normal(vector) => parser.normals.push(vector),
                Statement::Face(data) => {
                    if let Some(ref group) = current_group {
                        parser.add_face_to_group(group, &data);
                    }
                    parser.faces.push(data)
                }
                Statement::Group(name) => current_group = Some(name),
                Statement::None => (),
            }
        }
    }

    parser
}

fn parse_statement(line: String) -> Statement {
    let mut line = line.split_whitespace();

    match line.next() {
        Some("v") => parse_vertex(line),
        Some("vn") => parse_normal(line),
        Some("f") => parse_face(line),
        Some("g") => parse_group(line),
        _ => Statement::None
    }
}

fn parse_vertex(line: SplitWhitespace) -> Statement {
    parse_tuple(line, |ps| Statement::Vertex(points::new(ps[0], ps[1], ps[2])))
}

fn parse_normal(line: SplitWhitespace) -> Statement {
    parse_tuple(line, |ns| Statement::Normal(vectors::new(ns[0], ns[1], ns[2])))
}

fn parse_tuple<F>(line: SplitWhitespace, f: F) -> Statement
    where F: FnOnce(Vec<Real>) -> Statement {
    let mut components: Vec<Real> = vec![];

    for word in line {
        if let Ok(component) = word.parse::<Real>() {
            components.push(component);
        }
    }

    if components.is_empty() {
        Statement::None
    } else {
        f(components)
    }
}

fn parse_face(line: SplitWhitespace) -> Statement {
    let mut vs: Vec<usize> = vec![];
    let mut ns: Vec<usize> = vec![];

    fn parse_part<F>(part: &str, f: F) where F: FnOnce(usize) -> () {
        if let Ok(index) = part.parse::<usize>() {
            f(index)
        }
    }

    for word in line {
        let mut parts = word.split("/");

        if let Some(part) = parts.next() {
            parse_part(part, |i| vs.push(i));
        }

        // Ignores texture vertex
        parts.next();

        if let Some(part) = parts.next() {
            parse_part(part, |i| ns.push(i));
        }
    }

    if vs.is_empty() {
        Statement::None
    } else {
        Statement::Face(FaceData::new(vs, ns))
    }
}

fn parse_group(mut line: SplitWhitespace) -> Statement {
    match line.next() {
        None => Statement::None,
        Some(name) => Statement::Group(name.to_string())
    }
}

#[cfg(test)]
mod tests;