fn run_p1(input: &str) {
    let values: Vec<(&str, &str)> = input.lines().map(|l| l.split_once(" ").unwrap()).collect();

    let mut depth: u32 = 0;
    let mut pos: u32 = 0;

    for v in values {
        let command: &str = v.0;
        let amount: u32 = v.1.parse().unwrap();

        match command {
            "forward" => pos += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => panic!("wat")
        }

    }

    println!("Day2 p1 : {}", depth*pos);
}

fn run_p2(input: &str) {
    let values: Vec<(&str, &str)> = input.lines().map(|l| l.split_once(" ").unwrap()).collect();

    let mut depth: u32 = 0;
    let mut pos: u32 = 0;
    let mut aim: u32 = 0;

    for v in values {
        let command: &str = v.0;
        let amount: u32 = v.1.parse().unwrap();

        match command {
            "forward" => {
                pos += amount;
                depth += aim * amount;
            },
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => panic!("wat")
        }
    }

    println!("Day2 p2 : {}", depth*pos);
}

fn main() {
    run_p1(include_str!("input.txt"));
    run_p2(include_str!("input.txt"));
}