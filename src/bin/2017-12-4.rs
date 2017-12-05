extern crate adventofcode;

use adventofcode::{input_reader, Part};
use std::env;
use std::io::BufRead;

fn main() {
    let part = env::args()
        .nth(1)
        .expect("missing part")
        .parse::<Part>()
        .expect("invalid part");
    let input_file = env::args().nth(2).unwrap_or("input/2017/day-4.txt".into());

    let count = input_reader(input_file)
        .expect("failed to open file")
        .lines()
        .map(|line| line.expect("could not read line"))
        .filter(|line| {
            let numbers = line.split_whitespace().collect::<Vec<_>>();
            numbers.iter().enumerate().all(|(i, a)| {
                numbers.iter().skip(i + 1).all(|b| match part {
                    Part::One => a != b,
                    Part::Two => {
                        let mut a: Vec<char> = a.chars().collect();
                        a.sort();
                        let mut b: Vec<char> = b.chars().collect();
                        b.sort();
                        a != b
                    }
                })
            })
        })
        .count();
    println!("{}", count);
}
