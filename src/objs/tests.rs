use crate::objs;
use std::io::Error;

#[test]
fn test_ignoring_unrecognized_files() {
    let gibberish = b"There was a young lady named Bright
        who traveled much faster than light.
        She set out one day.
        in a relative way,
        and came back the previous night." as &[u8];
    let result = objs::parse(gibberish);
    match result {
        Ok(success) => panic!("Should be an error. Got {:?}", success),
        Err(errors) => assert_eq!(errors.len(), 28)
    }
}