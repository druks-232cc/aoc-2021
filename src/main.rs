use std::fs;

mod day1;
mod day2;
mod day3;

fn main() {
    let input = fs::read_to_string("input/day1.txt").expect("Error while reading");

    day1::run_p1(input.clone());
    day1::run_p2(input.clone());

    let input = fs::read_to_string("input/day2.txt").expect("Error while reading");

    day2::run_p1(input.clone());
    day2::run_p2(input.clone());

    let input = fs::read_to_string("input/day3.txt").expect("Error while reading");

    day3::run_p1(input.clone());
    day3::run_p2(input.clone());
}
