extern crate adventofcode;

use adventofcode::input_reader;
use std::env;
use std::io::BufRead;

fn main() {
    let input_file = env::args().nth(1).unwrap_or("input/2017/day-11.txt".into());

    let input = input_reader(input_file)
        .expect("could not open file")
        .lines()
        .collect::<Vec<_>>();

}


#[cfg(test)]
mod tests {
    #[test]
    fn test_hash() {
        //assert_eq!(0, least_steps(&vec![NorthEast, South, NorthWest]));
    }
}
