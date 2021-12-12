fn run_p1(input: &str) {
    let values: Vec<u32> = input.lines().map(|v| v.parse().unwrap()).collect();

    let mut prev = u32::MAX;
    let mut count = 0;

    for v in values {
        if v > prev {
            count += 1;
        }
        prev = v;
    }

    println!("Day1 p1 : {}", count);
}

fn run_p2(input: &str) {
    let values: Vec<u32> = input.lines().map(|v| v.parse().unwrap()).collect();

    let mut prev = u32::MAX;
    let mut count = 0;

    for i in 0 .. values.len()-2 {
        let sum: u32 = values[i..i+3].iter().sum();

        if sum > prev {
            count += 1;
        }
        prev = sum;
    }

    println!("Day1 p2 : {}", count);
}

fn main() {
    run_p1(include_str!("input.txt"));
    run_p2(include_str!("input.txt"));
}