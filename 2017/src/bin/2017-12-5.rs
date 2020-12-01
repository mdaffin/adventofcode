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
    let input_file = env::args().nth(2).unwrap_or("input/2017/day-5.txt".into());

    let mut array = input_reader(input_file)
        .expect("failed to open file")
        .lines()
        .map(|line| {
            line.expect("could not read line").parse().expect(
                "failed to parse input",
            )
        })
        .collect::<Vec<i32>>();

    let mut pos = 0;
    let mut counter = 0;
    loop {
        counter += 1;
        let cur = array[pos];
        array[pos] += match (&part, cur >= 3) {
            (&Part::Two, true) => -1,
            _ => 1,
        };
        let next = pos as i32 + cur;
        //println!("{:?} pos({}) cur({}) next({})", array, pos, cur, next);
        if next < 0 || next >= array.len() as i32 {
            break;
        }
        pos = next as usize;
    }
    println!("{}", counter);
}
