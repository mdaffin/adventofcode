#[macro_use]
extern crate nom;

extern crate adventofcode;

use adventofcode::input_reader;
use std::env;
use std::io::BufRead;

#[derive(Debug)]
struct Group {
    score: u32,
    children: Vec<Group>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            score: 1,
            children: Vec::new(),
        }
    }

    pub fn total_score(&self) -> u32 {
        self.score + self.children.iter().map(|c| c.total_score()).sum::<u32>()
    }

    fn inc_score(&mut self) {
        self.score += 1;
        for child in self.children.iter_mut() {
            child.inc_score()
        }
    }

    pub fn add(&mut self, mut child: Group) {
        child.inc_score();
        self.children.push(child)
    }
}

named!(garbage<u32>, do_parse!(
    tag!("<") >>
    count: fold_many0!(
        alt!(
            pair!(tag!("!"), take!(1)) => { |_| 0 } |
            is_not!(">!") => { |i: &[u8]| i.len() as u32 }
        ), 0, |acc, i| acc + i
    ) >>
    tag!(">") >>
    (count)
));

named!(group<(Option<Group>, u32)>, alt!(
        garbage => { |c| (None, c) } |
        map!(do_parse!(
            tag!("{") >>
            children: many0!(do_parse!(
                inner: group >>
                opt!(tag!(",")) >>
                (inner)
            )) >>
            tag!("}") >>
            (children)
        ), |children| {
            let mut g = Group::new();
            let count = children.into_iter().map(|(child, count)| {
                if let Some(child) = child {
                    g.add(child);
                }
                count
            }).sum();
            (Some(g), count)
        })
    )
);

fn main() {
    let input_file = env::args().nth(1).unwrap_or("input/2017/day-9.txt".into());
    for line in input_reader(input_file)
        .expect("failed to open file")
        .lines()
        .map(|line| line.expect("could not read line"))
    {
        match group(line.as_bytes()) {
            nom::IResult::Done(_, (None, _)) => {
                println!("No groups found");
            }
            nom::IResult::Done(_, (Some(groups), garbage_count)) => {
                println!("Groups: {} Garbage: {}", groups.total_score(), garbage_count);
            }
            nom::IResult::Error(err) => panic!("{:?}: {}", err, line),
            nom::IResult::Incomplete(_) => panic!("incomplete"),
        }
    }
}
