const INPUT: &'static str = include_str!("day03.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn number_of_trees_for(slope: &str, right: usize, down: usize) -> usize {
    let len = slope.lines().next().unwrap().chars().count();

    let mut x = 0;
    slope
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

fn part1(slope: &str) -> usize {
    number_of_trees_for(slope, 3, 1)
}

fn part2(slope: &str) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| number_of_trees_for(slope, *right, *down))
        .product()
}
