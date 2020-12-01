extern crate adventofcode;

use adventofcode::input_reader;
use std::env;
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]
enum Direction {
    North = 0,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
}

fn main() {
    let input_file = env::args().nth(1).unwrap_or("input/2017/day-11.txt".into());

    let input = input_reader(input_file)
        .expect("could not open file")
        .lines()
        .next()
        .expect("empty file")
        .expect("failed to read line");
    let input = input.split(",")
        .map(|d| match d {
            "n" => Direction::North,
            "nw" => Direction::NorthWest,
            "ne" => Direction::NorthEast,
            "s" => Direction::South,
            "sw" => Direction::SouthWest,
            "se" => Direction::SouthEast,
            _ => panic!("Invalid direction"),
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", least_steps(&input));

    let largest_disance = (0..input.len())
        .into_iter()
        .map(|i| least_steps(&input[0..i]))
        .max()
        .expect("no directions");
    println!("Part 2: {}", largest_disance);

}

fn least_steps(steps: &[Direction]) -> u32 {
    use Direction::*;
    let mut map = steps.iter().fold([0; 6], |mut m, &d| {
        m[d as usize] += 1;
        m
    });

    for _ in 0..2 {
        {
            // NW + S = SW
            let diff = std::cmp::min(map[NorthWest as usize], map[South as usize]);
            map[NorthWest as usize] -= diff;
            map[South as usize] -= diff;
            map[SouthWest as usize] += diff;
        }

        {
            // NE + S = SE
            let diff = std::cmp::min(map[NorthEast as usize], map[South as usize]);
            map[NorthEast as usize] -= diff;
            map[South as usize] -= diff;
            map[SouthEast as usize] += diff;
        }

        {
            // SW + N = NW
            let diff = std::cmp::min(map[SouthWest as usize], map[North as usize]);
            map[SouthWest as usize] -= diff;
            map[North as usize] -= diff;
            map[NorthWest as usize] += diff;
        }

        {
            // SE + N = NE
            let diff = std::cmp::min(map[SouthEast as usize], map[North as usize]);
            map[SouthEast as usize] -= diff;
            map[North as usize] -= diff;
            map[NorthEast as usize] += diff;
        }

        {
            // NW + NE = N
            let diff = std::cmp::min(map[NorthWest as usize], map[NorthEast as usize]);
            map[NorthWest as usize] -= diff;
            map[NorthEast as usize] -= diff;
            map[North as usize] += diff;
        }

        {
            // SW + SE = S
            let diff = std::cmp::min(map[SouthWest as usize], map[SouthEast as usize]);
            map[SouthWest as usize] -= diff;
            map[SouthEast as usize] -= diff;
            map[South as usize] += diff;
        }


        {
            // NW + SE = 0
            let diff = std::cmp::min(map[NorthWest as usize], map[SouthEast as usize]);
            map[NorthWest as usize] -= diff;
            map[SouthEast as usize] -= diff;
        }

        {
            // NE + SW = 0
            let diff = std::cmp::min(map[NorthEast as usize], map[SouthWest as usize]);
            map[NorthEast as usize] -= diff;
            map[SouthWest as usize] -= diff;
        }

        {
            // N + S = 0
            let diff = std::cmp::min(map[North as usize], map[South as usize]);
            map[North as usize] -= diff;
            map[South as usize] -= diff;
        }
    }

    map.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::Direction::*;
    use super::least_steps;

    #[test]
    fn test_hash() {
        assert_eq!(6,
                   least_steps(&vec![NorthEast, SouthEast, North, North, NorthWest, NorthWest,
                                     NorthWest, North, NorthWest, NorthWest, SouthWest,
                                     SouthWest, SouthWest, SouthWest, South, SouthEast, South]));
        assert_eq!(1, least_steps(&vec![NorthEast, NorthWest]));
        assert_eq!(3, least_steps(&vec![NorthEast, NorthEast, NorthEast]));
        assert_eq!(0,
                   least_steps(&vec![NorthEast, NorthEast, SouthWest, SouthWest]));
        assert_eq!(2, least_steps(&vec![NorthEast, NorthEast, South, South]));
        assert_eq!(3,
                   least_steps(&vec![SouthEast, SouthWest, SouthEast, SouthWest, SouthWest]));
        assert_eq!(0, least_steps(&vec![South, North]));
        assert_eq!(1, least_steps(&vec![NorthEast, South]));
        assert_eq!(0, least_steps(&vec![NorthEast, South, NorthWest]));
    }
}
