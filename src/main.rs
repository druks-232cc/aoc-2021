use std::fs;

mod day1;

fn main() {
    let input = fs::read_to_string("input/day1.txt").expect("Error while reading");

    day1::run_p1(input.clone());
    day1::run_p2(input.clone());
}
