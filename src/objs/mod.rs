use std::io::{BufReader, BufRead, Read};


use crate::math::Real;


use crate::tuples::points::{Point};
use crate::tuples::{points};
use crate::shapes::groups::Group;
use std::str::SplitWhitespace;
use crate::shapes::triangles::Triangle;
use std::rc::Rc;
use crate::shapes::{Shape, Geo};
use std::collections::HashMap;


pub struct Parser {
    vertices: Vec<Point>,
    faces: Vec<FaceData>,
    groups: HashMap<String, Rc<Shape>>,
}

pub enum Statement {
    Vertex(Point),
    Face(FaceData),
    Group(String),
    None,
}

type FaceData = Vec<usize>;

impl Parser {
    pub fn new(mut vertices: Vec<Point>, faces: Vec<FaceData>) -> Parser {
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
        self.group_faces(key, self.faces.clone());
        self.groups.get(key).map(|r| (*r).clone()).unwrap_or(Rc::new(Shape::empty_group()))
    }

    fn group_faces(&mut self, name: &str, faces: Vec<FaceData>) {
        faces.iter().for_each(|face| {
            self.add_face_to_group(name, face);
        })
    }

    fn add_face_to_group(&mut self, name: &str, face: &FaceData) -> Option<Rc<Shape>> {
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

        self.groups.insert(name.to_string(), group)
    }

    /// Converts polygons into triangles
    fn fan_triangulation(&self, face: &FaceData) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = vec![];
        for i in 1..face.len() - 1 {
            let triangle = Triangle::regular(
                self.vertices[face[0]],
                self.vertices[face[i]],
                self.vertices[face[i + 1]],
            );
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
    let mut parser = Parser::new(vec![], vec![]);

    for line in lines {
        if let Ok(line) = line {
            match parse_statement(line) {
                Statement::Vertex(point) => parser.vertices.push(point),
                Statement::Face(face) => {
                    if let Some(ref group) = current_group {
                        parser.add_face_to_group(group, &face);
                    }
                    parser.faces.push(face)
                }
                Statement::Group(name) => current_group = Some(name),
                _ => ()
            }
        }
    }
    parser
}

fn parse_statement(line: String) -> Statement {
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
        Some(name) => Statement::Group(name.to_string())
    }
}

#[cfg(test)]
mod tests;