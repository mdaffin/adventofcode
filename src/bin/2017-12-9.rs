#[macro_use]
extern crate nom;

extern crate adventofcode;

use std::str::{self, FromStr};
use adventofcode::input_reader;
use std::env;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let input_file = env::args().nth(1).unwrap_or("input/2017/day-8.txt".into());
}
