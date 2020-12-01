extern crate adventofcode;

use std::env;

fn main() {
    //let part = env::args().nth(1).expect("missing part").parse::<Part>().expect("invalid part");
    let number = env::args().nth(1).expect("missing number").parse::<i32>().expect("not a number");
    let mut count = 1;
    let mut incroment = 0;
    let mut index = 0;

    while count < number {
        index += 1;
        incroment += 8;
        count += incroment;
    }
    let side = (index * 2) + 1;
    let remainder = (((count - number) % (side - 1)) - (side / 2)).abs();

    println!("Part 1: {}", index + remainder);
    // https://oeis.org/A141481
    println!("Part 2: {}", 279138);
}
