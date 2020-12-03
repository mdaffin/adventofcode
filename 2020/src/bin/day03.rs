const INPUT: &'static str = include_str!("day03.txt");
/*
const INPUT: &'static str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
*/

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn check_slope(input: &str, right: usize, down: usize) -> usize {
    let len = input.lines().next().unwrap().chars().count();

    let mut x = 0;
    input
        .lines()
        .enumerate()
        .filter(|(index, _)| index % down == 0)
        .map(|(_, line)| {
            let cur_x = x;
            x = (x + right) % len;
            line.chars().nth(cur_x).unwrap()
        })
        .filter(|&c| c == '#')
        .count()
}

fn part1(input: &str) -> usize {
    check_slope(input, 3, 1)
}

fn part2(input: &str) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| check_slope(input, *right, *down))
        .product()
}
