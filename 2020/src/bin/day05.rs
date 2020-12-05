const INPUT: &'static str = include_str!("day05.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    input.lines().map(to_seat_id).max().unwrap()
}

fn part2(input: &str) -> u32 {
    let mut ids = input.lines().map(to_seat_id).collect::<Vec<_>>();
    ids.sort();
    let start = ids[0];
    let (index, _) = ids
        .into_iter()
        .enumerate()
        .map(|(i, id)| (i as u32 + start, id))
        .find(|&(i, id)| i != id)
        .unwrap();
    index
}

fn to_seat_id(key: &str) -> u32 {
    key.chars()
        .map(|c| match c {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => panic!("invalid character in ticket"),
        })
        .fold(0, |acc, n| (acc << 1) + n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn part1_actual() {
        assert_eq!(part1(INPUT), 801);
    }

    #[test]
    fn part2_actual() {
        assert_eq!(part2(INPUT), 597);
    }

    #[test_case("BFFFBBFRRR", 567)]
    #[test_case("FFFBBBFRRR", 119)]
    #[test_case("BBFFBBFRLL", 820)]
    fn decode_seat_seat_id(seat: &str, seat_id: u32) {
        assert_eq!(to_seat_id(seat), seat_id)
    }
}
