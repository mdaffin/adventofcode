use std::fs;
use std::env;
use std::io::{self, BufReader, Read, BufRead};

enum Part {
    One,
    Two,
}

fn main() {
    let part = match env::args().nth(1).expect("missing method, use `1` or `2`").as_ref() {
        "1" => Part::One,
        "2" => Part::Two,
        _ => panic!("unknown method, use `1` or `2`"),
    };
    let rdr: BufReader<Box<Read>> = BufReader::new(if let Some(input) = env::args().nth(2) {
        Box::new(fs::File::open(input).expect("could not open file"))
    } else {
        Box::new(io::stdin())
    });

    let cksum = rdr.lines().fold(0, |cksum, line| {
        let line = line.expect("could not read line");
        let iter = line.split_whitespace()
            .map(|s| s.parse::<u32>().expect("could not parse input"));
        match part {
            Part::One => {
                let (min, max) = iter.fold((None, None), |(min, max), i| {
                    (if i <= min.unwrap_or(i) { Some(i) } else { min },
                     if i >= max.unwrap_or(i) { Some(i) } else { max })
                });
                cksum + max.unwrap_or(0) - min.unwrap_or(0)
            }
            Part::Two => {
                let numbers = iter.collect::<Vec<_>>();
                let quotient = numbers.iter()
                    .enumerate()
                    .map(|(i, a)| {
                        numbers.iter()
                            .skip(i + 1)
                            .map(|b| if a % b == 0 {
                                Some(a / b)
                            } else if b % a == 0 {
                                Some(b / a)
                            } else {
                                None
                            })
                            .find(|q| q.is_some())
                            .unwrap_or(None)
                    })
                    .find(|q| q.is_some())
                    .unwrap_or(None);
                cksum + quotient.unwrap_or(0)
            }
        }
    });
    println!("{}", cksum);
}
