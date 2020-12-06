use itertools::Itertools;
const INPUT: &'static str = include_str!("day06.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let is_lowercase_alpha = |&c: &char| c >= 'a' && c <= 'z';

    input
        .split("\n\n")
        .map(|group| group.chars().filter(is_lowercase_alpha).unique().count())
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    const ALPHABET_LENGTH: usize = 26;

    let is_newline = |&c: &char| c == '\n';
    let group_answers = |mut answers: [_; ALPHABET_LENGTH], c| {
        answers[c as usize - 'a' as usize] += 1;
        answers
    };

    input
        .trim()
        .split("\n\n")
        .map(|group| {
            let number_of_people = group.trim().chars().filter(is_newline).count() + 1;
            let grouped_answers = group
                .chars()
                .filter(|c| !is_newline(c))
                .fold([0; ALPHABET_LENGTH], group_answers);
            grouped_answers
                .iter()
                .inspect(|&&count| assert!(count <= number_of_people))
                .filter(|&&count| count == number_of_people)
                .count()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n", 11)]
    fn part1_example(input: &str, sum: usize) {
        assert_eq!(part1(input), sum)
    }

    #[test]
    fn part1_actual() {
        assert_eq!(part1(INPUT), 6596);
    }

    #[test_case("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n", 6)]
    fn part_2_example(input: &str, sum: usize) {
        assert_eq!(part2(input), sum)
    }

    #[test]
    fn part2_actual() {
        assert_eq!(part2(INPUT), 3219);
    }
}
