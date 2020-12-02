use anyhow::Result;
use itertools::Itertools;

const INPUT: &'static str = include_str!("day01.txt");

fn main() -> Result<()> {
    let numbers = INPUT
        .lines()
        .map(|l| l.parse().expect("could not parse input"))
        .collect::<Vec<u32>>();
    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
    Ok(())
}

#[adventofcode::part("input/day01.txt")]
pub fn part1(numbers: &[u32]) -> u32 {
    let (n1, n2) = numbers
        .iter()
        .cartesian_product(numbers.iter())
        .find(|(n1, n2)| *n1 + *n2 == 2020)
        .unwrap();

    n1 * n2
}

#[test]
fn part1_test() {
    let numbers = INPUT
        .lines()
        .map(|l| l.parse().expect("could not parse input"))
        .collect::<Vec<u32>>();
    assert_eq!(part1(&numbers), 898299);
}

#[adventofcode::part("input/day01.txt")]
fn part2(numbers: &[u32]) -> u32 {
    let ((n1, n2), n3) = numbers
        .iter()
        .cartesian_product(numbers.iter())
        .cartesian_product(numbers.iter())
        .find(|((n1, n2), n3)| *n1 + *n2 + *n3 == 2020)
        .unwrap();

    n1 * n2 * n3
}

#[test]
fn part2_test() {
    let numbers = INPUT
        .lines()
        .map(|l| l.parse().expect("could not parse input"))
        .collect::<Vec<u32>>();
    assert_eq!(part2(&numbers), 143933922);
}
