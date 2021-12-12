use std::cmp::{min,max};

fn distance(a: u32, b: u32) -> u32 {
    max(a, b) - min(a, b)
}

fn run(input: &str) -> Option<()> {
    let mut crabs: Vec<u32> = input.lines().next()?
                                       .split(',')
                                       .map(|x| x.parse().unwrap())
                                       .collect();

    crabs.sort();

    let median = *crabs.iter().nth(crabs.len()/2)?;
    let mut fuel = 0;

    crabs.iter().for_each(|x| fuel += distance(median, *x));

    println!("Day7 p1 : {}", fuel);

    let mean: u32 = (crabs.iter().sum::<u32>() as f32 / crabs.len() as f32)
                    .round() as u32 - 1; // -1 needed ??? 474.586 rounded to 475
    fuel = 0;

    crabs.iter().for_each(|x|
        fuel += (distance(mean, *x))*
                (distance(mean, *x) + 1)
                /2); // n*(n+1)/2

    println!("Day7 p2 : {}", fuel);

    Some(())
}

fn main() {
    run(include_str!("input.txt"));
}