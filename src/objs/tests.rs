use crate::objs;
use std::io::Error;
use crate::tuples::points;

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
fn test_vertex_records() {
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