use anyhow::Result;

use combine::parser::char::*;
use combine::stream::easy;
use combine::*;

const INPUT: &'static str = include_str!("day02.txt");

fn main() -> Result<()> {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
    Ok(())
}

#[derive(Debug)]
struct Input {
    min: usize,
    max: usize,
    search_char: char,
    password: String,
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|i| line_parser(i))
        .map(|i| (i.min, i.max, i.password.matches(i.search_char).count()))
        .filter(|(min, max, count)| count >= min && count <= max)
        .count()
}
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|i| line_parser(i))
        .map(|i| (i.search_char, i.password.chars().nth(i.min - 1), i.password.chars().nth(i.max - 1)))
        .map(|(search_char, a, b)| (Some(search_char) == a, Some(search_char) == b))
        .filter(|(a, b)| a ^ b)
        .count()
}

fn line_parser(input: &str) -> Input {
    let integer =
        || spaces().with(many1(digit()).map(|string: String| string.parse::<usize>().unwrap()));
    let range = || spaces().with((integer(), char('-'), integer()).map(|(min, _, max)| (min, max)));
    let search_letter = || spaces().with((letter(), char(':')).map(|l| l.0));
    let password = || spaces().with(many1(letter()));

    let line = || {
        (range(), search_letter(), password()).map(|((min, max), search_char, password)| Input {
            min,
            max,
            search_char,
            password,
        })
    };

    let (line, remaining): (Input, &str) = line().easy_parse(input).unwrap();
    if remaining.len() > 0 {
        panic!("tailing input found")
    }
    line
}
