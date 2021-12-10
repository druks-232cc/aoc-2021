use std::collections::HashMap;

fn split_line(s: &str) -> ([&str;10], [&str;4]) {
    let (a, b) = s.split_once('|').unwrap();

    (a.split_whitespace().collect::<Vec<&str>>().try_into().unwrap(),
     b.split_whitespace().collect::<Vec<&str>>().try_into().unwrap())
}

fn findout_mapping(enc_numbers: &[&str; 10]) -> HashMap<char,u32> {
    let mut n = enc_numbers.clone();
    let mut h = HashMap::new();
    let mut h_count: HashMap<char, u32> = HashMap::new();

    n.sort_by(|a,b| a.len().cmp(&b.len()));
    n.iter().for_each(|s|
        s.chars().for_each(|c|
            *h_count.entry(c).or_insert(0) += 1));

    for (c, v) in h_count {
        if v == 4 {
            h.insert(c, 4);       // e == 4
        } else if v == 6 {
            h.insert(c, 32);       // b == 32
        } else if v == 7 {
            if n[2].contains(c) {
                h.insert(c, 8);   // d == 8
            } else {
                h.insert(c, 1);   // g == 1
            }
        } else if v == 8 {
            if n[0].contains(c) {
                h.insert(c, 16);   // c == 16
            } else {
                h.insert(c, 64);   // a == 64
            }
        } else if v == 9 {
            h.insert(c, 2);       // f == 2
        }
    }

    h
}

fn numeric_output(x: [&str; 4], mapping: &HashMap<char,u32>) -> u32 {
    let mut output = 0;
    let numbers: HashMap<u32,u32> = HashMap::from([
        (0b1110111,0),
        (0b0010010,1),
        (0b1011101,2),
        (0b1011011,3),
        (0b0111010,4),
        (0b1101011,5),
        (0b1101111,6),
        (0b1010010,7),
        (0b1111111,8),
        (0b1111011,9),
    ]);

    for (i, s) in x.iter().enumerate() {
        let mut word_value = 0;
        for c in s.chars() {
            word_value += mapping.get(&c).unwrap();
        }
        output += numbers.get(&word_value).unwrap()*(10u32.pow(3 - i as u32));
    }
    output
}

pub(crate) fn run(input: &str) -> Option<()> {
    let (signals, output): (Vec<[&str;10]>, Vec<[&str;4]>) =
            input.lines()
                 .map(split_line)
                 .unzip();

    let mut count = 0;

    output.iter().for_each(|x|
        x.iter().for_each(|x|
            if x.len().ne(&5) && x.len().ne(&6)
            {count += 1} ));

    println!("Day8 p1 : {}", count);

    count = 0;
    let mappings: Vec<HashMap<char,u32>>;

    mappings = signals.iter().map(findout_mapping).collect();
    output.iter().zip(mappings.iter()).for_each(|(x, mapping)| count += numeric_output(*x, mapping));

    println!("Day8 p2 : {}", count);

    Some(())
}