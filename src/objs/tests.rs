use crate::objs;
use std::io::Error;

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