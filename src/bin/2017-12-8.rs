#[macro_use]
extern crate nom;

extern crate adventofcode;

use std::str::{self, FromStr};
use adventofcode::input_reader;
use std::env;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    Inc,
    Dec,
}

#[derive(Debug)]
struct Function {
    reg: String,
    operation: Operation,
    value: i32,
}

#[derive(Debug)]
enum Comparison {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug)]
struct Condition {
    reg: String,
    comparison: Comparison,
    number: i32,
}

#[derive(Debug)]
struct Expression {
    function: Function,
    condition: Condition,
}

impl Condition {
    pub fn evaluate(&self, registers: &mut HashMap<String, i32>) -> bool {
        use Comparison::*;
        let value = *registers.entry(self.reg.to_string()).or_insert(0);
        match self.comparison {
            Equal => value == self.number,
            NotEqual => value != self.number,
            LessThan => value < self.number,
            LessThanOrEqual => value <= self.number,
            GreaterThan => value > self.number,
            GreaterThanOrEqual => value >= self.number,
        }
    }
}

impl Function {
    pub fn execute(&self, registers: &mut HashMap<String, i32>) -> i32 {
        use Operation::*;
        let value = registers.entry(self.reg.to_string()).or_insert(0);
        match self.operation {
            Inc => *value += self.value,
            Dec => *value -= self.value,
        };
        *value
    }
}

named!(expression<Expression>, do_parse!(
    function: function >>
    condition: condition >>
    (Expression{function, condition})
));

named!(number<i32>, map_res!(
    map_res!(
        recognize!(pair!(
            opt!(alt!(tag_s!("+") | tag_s!("-"))),  // maybe sign?
            nom::digit
        )),
        str::from_utf8
    ),
    FromStr::from_str
));

named!(function<Function>, do_parse!(
    reg: ws!(map_res!(nom::alpha, std::str::from_utf8)) >>
    operation: ws!(alt!(
        tag!("inc") => { |_| Operation::Inc } |
        tag!("dec") => { |_| Operation::Dec }
    )) >>
    value: number >>
    (Function { reg: reg.into(), operation, value })
));

named!(condition<Condition>, do_parse!(
    ws!(tag!("if")) >>
    reg: ws!(map_res!(nom::alpha, std::str::from_utf8)) >>
    comparison: ws!(comparison) >>
    number: ws!(number) >>
    (Condition{ reg: reg.into(), comparison, number })
));

named!(comparison<Comparison>, alt!(
    tag!("==") => { |_| Comparison::Equal } |
    tag!("!=") => { |_| Comparison::NotEqual } |
    tag!("<=") => { |_| Comparison::LessThanOrEqual } |
    tag!("<")  => { |_| Comparison::LessThan } |
    tag!(">=") => { |_| Comparison::GreaterThanOrEqual } |
    tag!(">")  => { |_| Comparison::GreaterThan }
));

fn main() {
    let input_file = env::args().nth(1).unwrap_or("input/2017/day-8.txt".into());
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut maximum = 0;

    for line in input_reader(input_file)
        .expect("failed to open file")
        .lines()
        .map(|line| line.expect("could not read line")) {
        match expression(line.as_bytes()) {
            nom::IResult::Done(_, exp) => {
                if exp.condition.evaluate(&mut registers) {
                    maximum = std::cmp::max(exp.function.execute(&mut registers), maximum);
                }
            }
            nom::IResult::Error(err) => panic!("{:?}: {}", err, line),
            nom::IResult::Incomplete(_) => panic!("incomplete"),
        }
    }

    println!("{:?}", registers);
    println!("Part 1: {}", registers.iter().map(|(_, v)| v).max().unwrap());
    println!("Part 2: {}", maximum);
}
