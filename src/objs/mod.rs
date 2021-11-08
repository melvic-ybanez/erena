use std::io::{Lines, BufReader, BufRead, Read, ErrorKind};
use std::fs::File;
use std::io;
use crate::math::Real;
use std::num::ParseFloatError;
use std::error::Error;

pub fn parse<R: Read>(file: R) -> Result<Vec<Real>, Vec<Box<dyn Error>>> {
    let lines = BufReader::new(file).lines();
    let mut success: Vec<Real> = vec![];
    let mut errors: Vec<Box<dyn Error>> = vec![];

    for line in lines {
        match line {
            Ok(line) => {
                line.split_whitespace().for_each(|word| {
                    match word.parse::<Real>() {
                        Ok(real) => success.push(real),
                        Err(err) => errors.push(Box::new(err))
                    }
                })
            },
            Err(error) => errors.push(Box::new(error))
        }
    }

    if !success.is_empty() {
        Ok(success)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests;