extern crate adventofcode;

use adventofcode::input_reader;
use std::env;
use std::io::BufRead;

const SIZE: usize = 256;

#[derive(Debug)]
struct CircularBuffer {
    inner: Vec<usize>,
}

impl CircularBuffer {
    pub fn new(size: usize) -> CircularBuffer {
        CircularBuffer { inner: (0..size).collect() }
    }

    pub fn iter(&self) -> std::iter::Cycle<std::slice::Iter<usize>> {
        self.inner.iter().cycle()
    }

    pub fn update(&mut self, pos: usize, values: &[usize]) {
        let mut pos = pos;
        for v in values {
            if pos >= self.inner.len() {
                pos = 0;
            }
            self.inner[pos] = *v;
            pos += 1;
        }
    }

    pub fn answer(&self) -> usize {
        self.inner[0] * self.inner[1]
    }
}

fn main() {
    let input_file = env::args().nth(1).unwrap_or("input/2017/day-10.txt".into());
    let lengths = input_reader(input_file)
        .expect("failed to open file")
        .lines()
        .map(|line| line.expect("could not read line"))
        .map(|line| {
            line.split(',')
                .map(|digits| digits.parse::<usize>().expect("not a number"))
                .collect::<Vec<_>>()
        })
        .next()
        .expect("missing input");

    let mut buffer = CircularBuffer::new(SIZE);
    let mut position: usize = 0;
    for (skip_size, length) in lengths.into_iter().enumerate() {
        let s = buffer
            .iter()
            .skip(position)
            .take(length)
            .map(|&v| v)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>();

        buffer.update(position, s.as_slice());
        position += skip_size + length;
        if position >= SIZE {
            position -= SIZE;
        }
    }
    println!("{}", buffer.answer());
}
