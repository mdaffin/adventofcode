use regex::Regex;
use std::str::FromStr;

const INPUT: &'static str = include_str!("day04.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[derive(Debug, Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl FromStr for Passport {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Passport, Self::Err> {
        s.split_whitespace().map(|s| s.split(":")).fold(
            Ok(Passport::default()),
            |acc, mut parts| {
                let mut acc = acc?;
                match (parts.next(), parts.next()) {
                    (Some("byr"), Some(value)) => acc.byr = Some(value.to_string()),
                    (Some("iyr"), Some(value)) => acc.iyr = Some(value.to_string()),
                    (Some("eyr"), Some(value)) => acc.eyr = Some(value.to_string()),
                    (Some("hgt"), Some(value)) => acc.hgt = Some(value.to_string()),
                    (Some("hcl"), Some(value)) => acc.hcl = Some(value.to_string()),
                    (Some("ecl"), Some(value)) => acc.ecl = Some(value.to_string()),
                    (Some("pid"), Some(value)) => acc.pid = Some(value.to_string()),
                    (Some("cid"), _) => (),
                    _ => return Err("invalid passport"),
                }
                Ok(acc)
            },
        )
    }
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(|i| i.parse::<Passport>().ok())
        .filter(|passport| {
            passport.byr.is_some()
                && passport.iyr.is_some()
                && passport.eyr.is_some()
                && passport.hgt.is_some()
                && passport.hcl.is_some()
                && passport.ecl.is_some()
                && passport.pid.is_some()
        })
        .count()
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
enum Height {
    #[display("{0}cm")]
    Centimeters(u32),
    #[display("{0}in")]
    Inches(u32),
}

fn part2(input: &str) -> usize {
    let is_between = |n: u32, min, max| n >= min && n <= max;
    let parse_number = |n: Option<&str>| n.unwrap_or("").parse::<u32>().unwrap_or(0);
    let hair_color = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let eye_color = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let passport_id = Regex::new(r"^[0-9]{9}$").unwrap();
    input
        .split("\n\n")
        .filter_map(|i| i.parse::<Passport>().ok())
        .filter(|p| is_between(parse_number(p.byr.as_deref()), 1920, 2002))
        .filter(|p| is_between(parse_number(p.iyr.as_deref()), 2010, 2020))
        .filter(|p| is_between(parse_number(p.eyr.as_deref()), 2020, 2030))
        .filter(|p| {
            match p
                .hgt
                .as_deref()
                .unwrap_or("")
                .parse::<Height>()
                .unwrap_or(Height::Inches(0))
            {
                Height::Centimeters(c) => is_between(c, 150, 193),
                Height::Inches(c) => is_between(c, 59, 76),
            }
        })
        .filter(|p| hair_color.is_match(p.hcl.as_deref().unwrap_or("")))
        .filter(|p| eye_color.is_match(p.ecl.as_deref().unwrap_or("")))
        .filter(|p| passport_id.is_match(p.pid.as_deref().unwrap_or("")))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    #[test]
    fn part1_actual() {
        assert_eq!(part1(INPUT), 210);
    }

    #[test]
    fn part2_actual() {
        assert_eq!(part2(INPUT), 131);
    }

    #[test]
    fn part1_example() {
        let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;
        assert_eq!(part1(input), 2);
    }

    #[test_case("pid:a hgt:a ecl:a iyr:a eyr:a byr:a hcl:a")]
    #[test_case("hgt:a ecl:a pid:a iyr:a eyr:a byr:a hcl:a")]
    #[test_case("ecl:a eyr:a pid:a iyr:a hgt:a byr:a hcl:a cid:a")]
    fn part1_valid_passports(input: &str) {
        assert_eq!(part1(input), 1);
    }

    #[test_case("hgt:a ecl:a iyr:a eyr:a byr:a hcl:a")]
    #[test_case("pid:a ecl:a iyr:a eyr:a byr:a hcl:a")]
    #[test_case("pid:a hgt:a iyr:a eyr:a byr:a hcl:a")]
    #[test_case("pid:a hgt:a ecl:a eyr:a byr:a hcl:a")]
    #[test_case("pid:a hgt:a ecl:a iyr:a byr:a hcl:a")]
    #[test_case("pid:a hgt:a ecl:a iyr:a eyr:a hcl:a")]
    #[test_case("pid:a hgt:a ecl:a iyr:a eyr:a byr:a")]
    #[test_case("a")]
    fn part1_invalid_passports(input: &str) {
        assert_eq!(part1(input), 0);
    }

    #[test_case("byr:1920 iyr:2010 eyr:2020 hgt:59in hcl:#000000 ecl:brn pid:000000000"; "lower_limit_in")]
    #[test_case("byr:2002 iyr:2020 eyr:2030 hgt:76in hcl:#ffffff ecl:amb pid:999999999"; "upper_limit_in")]
    #[test_case("byr:1920 iyr:2010 eyr:2020 hgt:150cm hcl:#000000 ecl:brn pid:000000000"; "lower_limit_cm")]
    #[test_case("byr:2002 iyr:2020 eyr:2030 hgt:193cm hcl:#ffffff ecl:amb pid:999999999"; "upper_limit_cm")]
    #[test_case("byr:1999 iyr:2017 eyr:2025 hgt:188cm hcl:#111111 ecl:grn pid:999999999"; "grn")]
    #[test_case("byr:1999 iyr:2017 eyr:2025 hgt:188cm hcl:#111111 ecl:amb pid:999999999"; "amb")]
    fn part2_valid_passports(input: &str) {
        assert_eq!(part2(input), 1);
    }

    #[test_case("hgt:a ecl:a iyr:a eyr:a byr:a hcl:a")]
    #[test_case("a")]
    #[test_case("byr:1919 iyr:2010 eyr:2020 hgt:59in hcl:#000000 ecl:brn pid:000000000")]
    #[test_case("byr:1920 iyr:2009 eyr:2020 hgt:59in hcl:#000000 ecl:brn pid:000000000")]
    #[test_case("byr:1920 iyr:2010 eyr:2019 hgt:59in hcl:#000000 ecl:brn pid:000000000")]
    #[test_case("byr:1920 iyr:2010 eyr:2020 hgt:58in hcl:#000000 ecl:brn pid:000000000")]
    #[test_case("byr:1920 iyr:2010 eyr:2020 hgt:149cm hcl:#000000 ecl:brn pid:000000000")]
    #[test_case("byr:2003 iyr:2020 eyr:2030 hgt:76in hcl:#ffffff ecl:amb pid:999999999")]
    #[test_case("byr:2002 iyr:2021 eyr:2030 hgt:76in hcl:#ffffff ecl:amb pid:999999999")]
    #[test_case("byr:2002 iyr:2020 eyr:2031 hgt:76in hcl:#ffffff ecl:amb pid:999999999")]
    #[test_case("byr:2002 iyr:2020 eyr:2030 hgt:77in hcl:#ffffff ecl:amb pid:999999999")]
    #[test_case("byr:2002 iyr:2020 eyr:2030 hgt:194cm hcl:#ffffff ecl:amb pid:999999999")]
    #[test_case("byr:2002 iyr:2020 eyr:2030 hgt:193cm hcl:#fffffff ecl:amb pid:999999999")]
    #[test_case("byr:2002 iyr:2020 eyr:2030 hgt:193cm hcl:#ffffgf ecl:amb pid:999999999")]
    #[test_case("byr:2002 iyr:2020 eyr:2030 hgt:193cm hcl:#ffffff ecl:ama pid:999999999")]
    #[test_case("byr:2002 iyr:2020 eyr:2030 hgt:193cm hcl:#ffffgf ecl:amb pid:a99999999")]
    fn part2_invalid_passports(input: &str) {
        assert_eq!(part2(input), 0);
    }
}
