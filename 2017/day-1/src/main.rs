use std::fs;
use std::env;
use std::io::{self, BufReader, Read, BufRead};
use std::ops::Fn;

fn main() {
    let next_index: Box<Fn(usize, usize) -> usize> =
        match env::args().nth(1).expect("missing method, use `1` or `2`").as_ref() {
            "1" => Box::new(|i, len| (i + 1) % len),
            "2" => Box::new(|i, len| (i + len / 2) % len),
            _ => panic!("unknown method, use `1` or `2`"),
        };
    let rdr: BufReader<Box<Read>> = BufReader::new(if let Some(input) = env::args().nth(2) {
        Box::new(fs::File::open(input).expect("could not open file"))
    } else {
        Box::new(io::stdin())
    });

    for line in rdr.lines() {
        let line = line.expect("could not read line");
        let digits =
            line.chars().map(|c| c.to_digit(10).expect("non digit found")).collect::<Vec<_>>();

        let acc = digits.iter()
            .enumerate()
            .map(|(i, d)| (next_index(i, digits.len()), d))
            .filter(|&(i, d)| &digits[i] == d)
            .fold(0, |acc, (_, d)| acc + d);

        println!("{}", acc);
    }
}
