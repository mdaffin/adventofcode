use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("day01.txt").unwrap();
    let numbers = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    part1(&numbers);
    part2(&numbers);
}

fn part1(numbers: &[u32]) {
    let (n1, n2) = numbers
        .iter()
        .cartesian_product(numbers.iter())
        .find(|(n1, n2)| *n1 + *n2 == 2020)
        .unwrap();

    println!("{}", n1 * n2);
}

fn part2(numbers: &[u32]) {
    let ((n1, n2), n3) = numbers
        .iter()
        .cartesian_product(numbers.iter())
        .cartesian_product(numbers.iter())
        .find(|((n1, n2), n3)| *n1 + *n2 + *n3 == 2020)
        .unwrap();

    println!("{}", n1 * n2 * n3);
}
