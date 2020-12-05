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
    let row = to_number(&key[..7], 'F');
    let column = to_number(&key[7..], 'L');
    row * 8 + column
}

fn to_number(key: &str, zero: char) -> u32 {
    key.chars()
        .rev()
        .map(|c| if c == zero { 0 } else { 1 })
        .enumerate()
        .map(|(i, c)| c << i)
        .sum()
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

    #[test_case("BFFFBBF", 70)]
    #[test_case("FFFBBBF", 14)]
    #[test_case("BBFFBBF", 102)]
    fn decode_seat_row(seat: &str, row: u32) {
        assert_eq!(to_number(seat, 'F'), row)
    }

    #[test_case("RRR", 7)]
    #[test_case("LLL", 0)]
    #[test_case("RLL", 4)]
    fn decode_seat_column(seat: &str, column: u32) {
        assert_eq!(to_number(seat, 'L'), column)
    }

    #[test_case("BFFFBBFRRR", 567)]
    #[test_case("FFFBBBFRRR", 119)]
    #[test_case("BBFFBBFRLL", 820)]
    fn decode_seat_seat_id(seat: &str, seat_id: u32) {
        assert_eq!(to_seat_id(seat), seat_id)
    }
}
