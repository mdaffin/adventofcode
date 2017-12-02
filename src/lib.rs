extern crate failure;
#[macro_use]
extern crate failure_derive;

use std::path::Path;
use std::fs::File;
use std::str::FromStr;
use std::io::{self, BufReader, Read};

#[derive(Fail, Debug)]
pub enum ParsePartError {
    #[fail(display = "{} is not a valid part.", _0)]
    InvalidPartNumber(String),
}

pub enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = ParsePartError;
    fn from_str(s: &str) -> Result<Self, ParsePartError> {
        match s.as_ref() {
            "1" | "one" => Ok(Part::One),
            "2" | "two" => Ok(Part::Two),
            i => Err(ParsePartError::InvalidPartNumber(i.to_string())),
        }
    }
}

pub fn input_reader<P>(filename: P) -> Result<BufReader<Box<Read>>, io::Error>
    where P: AsRef<Path>
{
    Ok(if filename.as_ref() == Path::new("-") {
        BufReader::new(Box::new(io::stdin()))
    } else {
        BufReader::new(Box::new(File::open(filename)?))
    })
}
