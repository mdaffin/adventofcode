extern crate adventofcode;

use adventofcode::{input_reader, Part};
use std::io::BufRead;
use std::env;

fn main() {
    let part = env::args().nth(1).expect("missing part").parse::<Part>().expect("invalid part");
    let input_file = env::args().nth(2).unwrap_or("input/2017/day-1.txt".into());

    for line in input_reader(input_file.clone())
        .expect(&format!("failed to open file `{}`", input_file))
        .lines() {
        let line = line.expect("could not read line");
        let digits =
            line.chars().map(|c| c.to_digit(10).expect("non digit found")).collect::<Vec<_>>();

        let acc = digits.iter()
            .enumerate()
            .map(|(i, d)| {
                (match part {
                     Part::One => (i + 1) % digits.len(),
                     Part::Two => (i + digits.len() / 2) % digits.len(),
                 },
                 d)
            })
            .filter(|&(i, d)| &digits[i] == d)
            .fold(0, |acc, (_, d)| acc + d);

        println!("{}", acc);
    }
}
