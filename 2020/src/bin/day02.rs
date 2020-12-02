const INPUT: &'static str = include_str!("day02.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display("{min}-{max} {letter}: {password}")]
struct Input {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|i| i.parse::<Input>().unwrap())
        .map(|i| (i.min, i.max, i.password.matches(i.letter).count()))
        .filter(|(min, max, count)| count >= min && count <= max)
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|i| i.parse::<Input>().unwrap())
        .map(|i| (i.letter, i.password.chars().nth(i.min - 1), i.password.chars().nth(i.max - 1)))
        .map(|(letter, a, b)| (Some(letter) == a, Some(letter) == b))
        .filter(|(a, b)| a ^ b)
        .count()
}
