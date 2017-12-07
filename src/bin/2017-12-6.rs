extern crate adventofcode;

use adventofcode::input_reader;
use std::env;
use std::io::BufRead;

fn main() {
    let input_file = env::args().nth(2).unwrap_or("input/2017/day-6.txt".into());

    let mut array = input_reader(input_file)
        .expect("failed to open file")
        .lines()
        // only take the first line - rest are ignored
        .next()
        .expect("no input")
        .expect("could not read input")
        .split_whitespace()
        .map(|s| s.parse().expect(&format!("failed to parse input: {}", s)))
        .collect::<Vec<u32>>();
    let mut previous = Vec::new();

    let diff;
    loop {
        previous.push(array.to_vec());

        // Find highest position
        let (mut i, &max) = array
            .iter()
            .enumerate()
            // Note: we cannot use .max() as it finds the last highest element were we need the
            // first.
            .fold(None, |max, (i, e)| match max {
                Some((_, me)) if e <= me => max,
                _ => Some((i, e)),
            })
            .expect("no numbers found");

        array[i] = 0;

        for _ in 0..max {
            i += 1;
            if i >= array.len() {
                i = 0;
            }
            array[i] += 1;
        }
        if let Some(index) = previous.iter().position(|a| a == &array) {
            diff = previous.len() - index;
            break;
        }
    }

    println!("Part 1: {}", previous.len());
    println!("Part 2: {}", diff);
}
