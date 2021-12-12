fn run_p1(input: &str) {
    let values: Vec<&str> = input.lines().collect();

    let mut gamma: u32 = 0;

    for i in 0 .. values[0].len() {
        let mut sum = 0;

        for v in &values {
            sum += v.chars().nth(i).unwrap().to_digit(10).unwrap();
        }
        if sum > (values.len() as u32)/2 {
            gamma += 2u32.pow((values[0].len() - i - 1) as u32);
        }
    }

    let epsilon: u32 = gamma ^ (2u32.pow(values[0].len() as u32)-1);

    println!("Day3 p1 : {}", gamma*epsilon);
}

fn run_p2(input: &str) {
    let mut v_o2: Vec<&str> = input.lines().collect();
    let mut v_co2 = v_o2.clone();

    for i in 0 .. v_o2[0].len() {
        let mut sum = 0;

        for v in &v_o2 {
            sum += v.chars().nth(i).unwrap().to_digit(10).unwrap();
        }
        if sum * 2  >= v_o2.len() as u32 {
            v_o2.retain(|x| x.chars().nth(i).unwrap() == '1');
        } else {
            v_o2.retain(|x| x.chars().nth(i).unwrap() == '0');
        }
        if v_o2.len() == 1 {
            break;
        }
    }

    for i in 0 .. v_co2[0].len() {
        let mut sum = 0;

        for v in &v_co2 {
            sum += v.chars().nth(i).unwrap().to_digit(10).unwrap();
        }
        if sum * 2 >= v_co2.len() as u32 {
            v_co2.retain(|x| x.chars().nth(i).unwrap() == '0');
        } else {
            v_co2.retain(|x| x.chars().nth(i).unwrap() == '1');
        }
        if v_co2.len() == 1 {
            break;
        }
    }

    let o2 = u32::from_str_radix(v_o2.first().unwrap(), 2).unwrap();
    let co2 = u32::from_str_radix(v_co2.first().unwrap(), 2).unwrap();

    println!("Day3 p2 : {}", o2*co2);
}

fn main() {
    run_p1(include_str!("input.txt"));
    run_p2(include_str!("input.txt"));
}