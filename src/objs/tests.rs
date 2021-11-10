use crate::objs;
use std::io::Error;
use crate::tuples::points;
use crate::shapes::Geo;
use std::fs::File;
use std::io;

#[test]
fn test_ignoring_unrecognized_files() {
    let gibberish = b"There was a young lady named Bright
        who traveled much faster than light.
        She set out one day.
        in a relative way,
        and came back the previous night." as &[u8];
    let result = objs::parse_obj(gibberish);
    assert!(result.is_empty());
}

#[test]
fn test_parsing_vertex_records() {
    let file = b"
        v -1 1 0
        v -1.0000 0.5000 0.0000
        v 1 0 0
        v 1 1 0" as &[u8];
    let parser = objs::parse_obj(file);
    let vertices = parser.get_vertices();

    assert_eq!(vertices[1], points::new(-1.0, 1.0, 0.0));
    assert_eq!(vertices[2], points::new(-1.0, 0.5, 0.0));
    assert_eq!(vertices[3], points::new(1.0, 0.0, 0.0));
    assert_eq!(vertices[4], points::new(1.0, 1.0, 0.0));
}

#[test]
fn test_parsing_faces() {
    let file = b"
        v -1 1 0
        v -1 0 0
        v 1 0 0
        v 1 1 0

        f 1 2 3
        f 1 3 4" as &[u8];
    let mut parser = objs::parse_obj(file);
    let group = parser.default_group();
    if let Geo::Group(g) = &group.geo {
        let t1 = g.get_child(0);
        let t2 = g.get_child(1);

        if let (Geo::Triangle(t1), Geo::Triangle(t2)) = (t1.geo, t2.geo) {
            let vertices = parser.get_vertices();

            assert_eq!(t1.get_p1(), vertices[1]);
            assert_eq!(t1.get_p2(), vertices[2]);
            assert_eq!(t1.get_p3(), vertices[3]);

            assert_eq!(t2.get_p1(), vertices[1]);
            assert_eq!(t2.get_p2(), vertices[3]);
            assert_eq!(t2.get_p3(), vertices[4]);
        }
    }
}

#[test]
fn test_triangulating_polygons() {
    let file = b"
        v -1 1 0
        v -1 0 0
        v 1 0 0
        v 1 1 0
        v 0 2 0
        f 1 2 3 4 5" as &[u8];
    let mut parser = objs::parse_obj(file);
    let group = parser.default_group();
    if let Geo::Group(g) = &group.geo {
        let t1 = g.get_child(0);
        let t2 = g.get_child(1);
        let t3 = g.get_child(2);

        if let (Geo::Triangle(t1), Geo::Triangle(t2), Geo::Triangle(t3)) = (t1.geo, t2.geo, t3.geo) {
            let vertices = parser.get_vertices();

            assert_eq!(t1.get_p1(), vertices[1]);
            assert_eq!(t1.get_p2(), vertices[2]);
            assert_eq!(t1.get_p3(), vertices[3]);

            assert_eq!(t2.get_p1(), vertices[1]);
            assert_eq!(t2.get_p2(), vertices[3]);
            assert_eq!(t2.get_p3(), vertices[4]);

            assert_eq!(t3.get_p1(), vertices[1]);
            assert_eq!(t3.get_p2(), vertices[4]);
            assert_eq!(t3.get_p3(), vertices[5]);
        }
    }
}

#[test]
fn test_triangles_in_groups() -> io::Result<()> {
    let file = File::open("resources/triangles.obj")?;
    let parser = objs::parse_obj(file);

    let t1 = parser.get_triangle_unsafe("FirstGroup", 0);
    let t2 = parser.get_triangle_unsafe("SecondGroup", 0);

    assert_eq!(t1.get_p1(), parser.vertices[1]);
    assert_eq!(t1.get_p2(), parser.vertices[2]);
    assert_eq!(t1.get_p3(), parser.vertices[3]);

    assert_eq!(t2.get_p1(), parser.vertices[1]);
    assert_eq!(t2.get_p2(), parser.vertices[3]);
    assert_eq!(t2.get_p3(), parser.vertices[4]);
    Ok(())
}