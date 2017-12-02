extern crate adventofcode;

use adventofcode::{Part, input_reader};
use std::env;
use std::io::BufRead;

fn main() {
    let part = env::args().nth(1).expect("missing part").parse::<Part>().expect("invalid part");
    let input_file = env::args().nth(2).unwrap_or("input/2017/day-2.txt".into());

    let cksum = input_reader(input_file)
        .expect("failed to open file")
        .lines()
        .fold(0, |cksum, line| {
            let line = line.expect("could not read line");
            let digits_iter = line.split_whitespace()
                .map(|s| s.parse::<u32>().expect("could not parse input"));
            cksum +
            match part {
                Part::One => {
                    let (min, max) = digits_iter.fold((None, None), |(min, max), i| {
                        (if i <= min.unwrap_or(i) { Some(i) } else { min },
                         if i >= max.unwrap_or(i) { Some(i) } else { max })
                    });
                    max.unwrap_or(0) - min.unwrap_or(0)
                }
                Part::Two => {
                    // Collect the digits so we can iter over the vector multiple times
                    let numbers = digits_iter.collect::<Vec<_>>();
                    numbers.iter()
                        .enumerate()
                        .map(|(i, a)| {
                            // Check a agienst the rest of the list
                            numbers.iter()
                                .skip(i + 1)
                                .map(|b| match (a % b, b % a) {
                                    (0, _) => Some(a / b),
                                    (_, 0) => Some(b / a),
                                    _ => None,
                                })
                                .find(|q| q.is_some())
                                .unwrap_or(None)
                        })
                        .find(|q| q.is_some())
                        .unwrap_or(None)
                        .unwrap_or(0)
                }
            }
        });
    println!("{}", cksum);
}
