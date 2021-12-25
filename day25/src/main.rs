use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(usize, usize);

fn parse_input(input: &str) -> (HashSet<Point>, HashSet<Point>, usize, usize) {
    let mut e_h = HashSet::new();
    let mut s_h = HashSet::new();
    let mut x_max = 0;
    let mut y_max = 0;

    for (j, l) in input.lines().enumerate() {
        for (i, c) in l.chars().enumerate() {
            match c {
                '>' => e_h.insert(Point(i,j)),
                'v' => s_h.insert(Point(i,j)),
                _ => false,
            };
            x_max = i;
        }
        y_max = j;
    }

    (e_h, s_h, x_max, y_max)
}

fn step_once(h1: HashSet<Point>, h2: HashSet<Point>, x_max: usize, y_max: usize) -> (HashSet<Point>, HashSet<Point>) {
    let mut nh1 = h1.clone();
    let mut nh2 = h2.clone();

    nh1.clear();
    nh2.clear();

    for e in &h1 {
        let new_point= Point((e.0+1)%(x_max+1),e.1);
        if h1.contains(&new_point) || h2.contains(&new_point) {
            nh1.insert(*e);
        } else {
            nh1.insert(new_point);
        }
    }

    for e in &h2 {
        let new_point= Point(e.0,(e.1+1)%(y_max+1));
        if nh1.contains(&new_point) || h2.contains(&new_point) {
            nh2.insert(*e);
        } else {
            nh2.insert(new_point);
        }
    }

    (nh1,nh2)
}

fn step_until(h1: HashSet<Point>, h2: HashSet<Point>, x_max: usize, y_max: usize) -> usize {
    let mut last_h1 = h1;
    let mut last_h2 = h2;
    let mut n = 0;

    loop {
        n = n + 1;
        let (new_h1, new_h2) = step_once(last_h1.clone(), last_h2.clone(), x_max, y_max);

        if new_h1 == last_h1 && new_h2 == last_h2 {
            break;
        } else {
            last_h1 = new_h1;
            last_h2 = new_h2;
        }
    }

    n
}

fn run(input: &str) -> Option<usize> {
    let (e_h, s_h, x, y) = parse_input(input);

    let p1 = step_until(e_h, s_h, x, y);

    Some(p1)
}

fn main() {
    let p1= run(include_str!("input.txt")).unwrap();
    println!("Day25 : {}", p1);
}

#[test]
fn test_input() {
    let p1 = run(include_str!("input.txt")).unwrap();
    assert_eq!(489, p1);
}

#[test]
fn test_input_sample_1() {
    let p1 = run(include_str!("input_sample_1.txt")).unwrap();
    assert_eq!(58, p1);
}