use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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

    let input = fs::read_to_string("input/day4.txt").expect("Error while reading");

    day4::run(input.clone());
    
    // Lets change things a bit
    let input= include_str!("../input/day5.txt");
    day5::run(input);
    
    let input= include_str!("../input/day6.txt");
    day6::run(input);

}
