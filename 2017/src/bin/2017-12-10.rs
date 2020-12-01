extern crate adventofcode;

use adventofcode::{input_reader, Part};
use std::env;
use std::io::BufRead;
use std::io::Read;

const SIZE: usize = 256;

struct KnotHash {
    buffer: [u8; SIZE],
    position: usize,
    skip_size: usize,
}

impl KnotHash {
    pub fn new() -> KnotHash {
        let mut buf = [0; SIZE];
        for i in 0..SIZE {
            buf[i] = i as u8;
        }

        KnotHash {
            buffer: buf,
            position: 0,
            skip_size: 0,
        }
    }

    fn iter(&self) -> std::iter::Cycle<std::slice::Iter<u8>> {
        self.buffer.iter().cycle()
    }

    fn update(&mut self, values: &[u8]) {
        let mut pos = self.position;
        for v in values {
            if pos >= self.buffer.len() {
                pos = 0;
            }
            self.buffer[pos] = *v;
            pos += 1;
        }
    }

    pub fn round(&mut self, input: &[u8]) {
        for length in input.iter().map(|i| *i as usize) {
            let s = self.iter()
                .skip(self.position)
                .take(length)
                .map(|&v| v)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>();

            self.update(s.as_slice());
            self.position += (self.skip_size + length) % SIZE;
            if self.position >= SIZE {
                self.position -= SIZE;
            }
            self.skip_size += 1;
        }
    }

    pub fn dense_hash(&self) -> [u8; 16] {
        let mut buf = [0; 16];
        for i in 0..buf.len() {
            buf[i] = self.buffer[i * 16..i * 16 + 16].iter().fold(
                0,
                |acc, i| acc ^ i,
            );
        }
        buf
    }

    pub fn part_one(&self) -> u32 {
        self.buffer[0] as u32 * self.buffer[1] as u32
    }
}

fn main() {
    let part = env::args()
        .nth(1)
        .expect("missing part")
        .parse::<Part>()
        .expect("invalid part");
    let input_file = env::args().nth(2).unwrap_or("input/2017/day-10.txt".into());

    match part {
        Part::One => part_one(&input_file),
        Part::Two => part_two(&input_file),
    }
}

fn part_one(input_file: &str) {
    let lengths = input_reader(input_file)
        .expect("failed to open file")
        .lines()
        .map(|line| line.expect("could not read line"))
        .map(|line| {
            line.split(',')
                .map(|digits| digits.parse::<u8>().expect("not a number"))
                .collect::<Vec<_>>()
        })
        .next()
        .expect("missing input");

    let mut hasher = KnotHash::new();
    hasher.round(&lengths);
    println!("{}", hasher.part_one());
}

fn part_two(input_file: &str) {
    let mut input = Vec::new();
    input_reader(input_file)
        .expect("failed to open file")
        .read_to_end(&mut input)
        .expect("failed to read file");
    if let Some(&10) = input.last() {
        input.pop();
    }
    println!("{}", hash(input));
}

fn hash(mut input: Vec<u8>) -> String {
    let mut hasher = KnotHash::new();
    input.append(&mut vec![17, 31, 73, 47, 23]);

    for _ in 0..64 {
        hasher.round(&input);
    }

    let mut output = String::new();
    for &byte in hasher.dense_hash().iter() {
        output += &format!("{:02x}", byte);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::hash;

    #[test]
    fn test_hash() {
        assert_eq!(
            "a2582a3a0e66e6e86e3812dcb672a272".to_string(),
            hash("".as_bytes().to_vec())
        );
        assert_eq!(
            "33efeb34ea91902bb2f59c9920caa6cd".to_string(),
            hash("AoC 2017".as_bytes().to_vec())
        );
        assert_eq!(
            "3efbe78a8d82f29979031a4aa0b16a9d".to_string(),
            hash("1,2,3".as_bytes().to_vec())
        );
        assert_eq!(
            "63960835bcdc130f0b66d7ff4f6a5a8e".to_string(),
            hash("1,2,4".as_bytes().to_vec())
        );
    }
}
