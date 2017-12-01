use std::fs;
use std::env;
use std::io::{self, BufReader, Read, BufRead};

fn main() {
    let rdr: BufReader<Box<Read>> = BufReader::new(if let Some(input) = env::args().nth(1) {
        Box::new(fs::File::open(input).expect("could not open file"))
    } else {
        Box::new(io::stdin())
    });

    for line in rdr.lines() {
        let line = line.expect("could not read line");
        let mut line = line.chars().peekable();
        let first = line.peek().expect("not enough input").clone();

        let mut acc = 0;
        while let Some(cur) = line.next() {
            let next = match line.peek() {
                Some(v) => v,
                None => &first,
            };
            if &cur == next {
                acc += cur.to_digit(10).expect("non digit found");
            }
        }
        println!("{}", acc);
    }
}
