use std::str::FromStr;
use thiserror::Error;

const INPUT: &'static str = include_str!("day04.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[derive(Error, Debug)]
enum PassportParseError {
    #[error("missing key")]
    MissingKey(&'static str),
    #[error("empty key")]
    EmptyKey,
    #[error("empty value")]
    EmptyValue,
    #[error("invalid key: {0}")]
    InvalidKey(String),
}

#[derive(Debug, Default)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display("{0}")]
struct Year(u32);

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
enum Height {
    #[display("{0}cm")]
    Centimeters(u32),
    #[display("{0}in")]
    Inches(u32),
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[from_str(regex = "#(?P<0>[0-9a-f]{6})")]
struct HairColor(String);

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
enum EyeColor {
    #[display("amb")]
    Amber,
    #[display("blu")]
    Blue,
    #[display("brn")]
    Brown,
    #[display("gry")]
    Gray,
    #[display("grn")]
    Green,
    #[display("hzl")]
    Hazel,
    #[display("oth")]
    Other,
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[from_str(regex = "(?P<0>[0-9]{9})")]
struct PassportID(String);

impl FromStr for Passport {
    type Err = PassportParseError;
    fn from_str(s: &str) -> Result<Passport, Self::Err> {
        let passport = s
            .split_whitespace()
            .map(|s| s.split(":"))
            .map(|mut s| (s.next(), s.next()))
            .fold(Ok(Passport::default()), |acc, (key, value)| {
                let mut acc = acc?;
                match (key, value) {
                    (None, _) => return Err(PassportParseError::EmptyKey),
                    (_, None) => return Err(PassportParseError::EmptyValue),
                    (Some("byr"), Some(value)) => acc.byr = value.to_string(),
                    (Some("iyr"), Some(value)) => acc.iyr = value.to_string(),
                    (Some("eyr"), Some(value)) => acc.eyr = value.to_string(),
                    (Some("hgt"), Some(value)) => acc.hgt = value.to_string(),
                    (Some("hcl"), Some(value)) => acc.hcl = value.to_string(),
                    (Some("ecl"), Some(value)) => acc.ecl = value.to_string(),
                    (Some("pid"), Some(value)) => acc.pid = value.to_string(),
                    (Some("cid"), Some(value)) => acc.cid = Some(value.to_string()),
                    (Some(key), _) => return Err(PassportParseError::InvalidKey(key.to_string())),
                }
                Ok(acc)
            })?;
        if passport.byr.is_empty() {
            Err(PassportParseError::MissingKey("byr"))
        } else if passport.iyr.is_empty() {
            Err(PassportParseError::MissingKey("iyr"))
        } else if passport.eyr.is_empty() {
            Err(PassportParseError::MissingKey("eyr"))
        } else if passport.hgt.is_empty() {
            Err(PassportParseError::MissingKey("hgt"))
        } else if passport.hcl.is_empty() {
            Err(PassportParseError::MissingKey("hcl"))
        } else if passport.ecl.is_empty() {
            Err(PassportParseError::MissingKey("ecl"))
        } else if passport.pid.is_empty() {
            Err(PassportParseError::MissingKey("pid"))
        } else {
            Ok(passport)
        }
    }
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Passport::from_str)
        .filter(|r| r.is_ok())
        .count()
}

fn part2(input: &str) -> usize {
    let is_between = |number: &str, min, max| {
        number
            .parse::<u32>()
            .map(|y| y >= min && y <= max)
            .unwrap_or(false)
    };
    input
        .split("\n\n")
        .map(Passport::from_str)
        .filter_map(|r| r.ok())
        .filter(|p| is_between(dbg!(&p.byr), 1920, 2002))
        .filter(|p| is_between(&p.iyr, 2010, 2020))
        .filter(|p| is_between(&p.eyr, 2020, 2030))
        .filter(|p| match p.hgt.parse::<Height>() {
            Ok(Height::Centimeters(c)) => c >= 150 && c <= 193,
            Ok(Height::Inches(c)) => c >= 59 && c <= 76,
            _ => false,
        })
        .filter(|p| p.hcl.parse::<HairColor>().is_ok())
        .filter(|p| p.ecl.parse::<EyeColor>().is_ok())
        .filter(|p| p.pid.parse::<PassportID>().is_ok())
        .count()
}
