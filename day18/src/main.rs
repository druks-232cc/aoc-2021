use std::{panic, str::Chars};

#[derive(Debug, Clone)]
struct Number {
    n: Vec<(u64, usize)>,
}

impl Number {
    // explosions are always a pair of two values
    fn explode(&mut self) -> bool {
        for (i, n) in self.n.clone().iter().enumerate() {
            if n.1 > 4 {
                // 1
                let v = self.n.remove(i).0;
                if i > 0 {
                    self.n.get_mut(i - 1).unwrap().0 += v
                }
                // 2
                let v = self.n.get(i).unwrap().0;
                if i + 1 <= self.n.len() - 1 {
                    self.n.get_mut(i + 1).unwrap().0 += v
                }
                *self.n.get_mut(i).unwrap() = (0, n.1 - 1);
                return true;
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        for (i, n) in self.n.clone().iter().enumerate() {
            if n.0 >= 10 {
                *self.n.get_mut(i).unwrap() = (n.0 / 2, n.1 + 1);
                self.n.insert(i + 1, (n.0 / 2 + n.0 % 2, n.1 + 1));
                return true;
            }
        }
        false
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn magnitude(&mut self) -> u64 {
        let mut values = self.n.clone();
        for i in (1..=4).rev() {
            let mut new_values = vec![];
            let mut skip_next = false;
            for (j, n) in values.clone().iter().enumerate() {
                if skip_next == true {
                    skip_next = false;
                    continue;
                } else if n.1 == i {
                    skip_next = true;
                    let v = values.get(j + 1).unwrap().0;
                    new_values.push((3 * n.0 + 2 * v, n.1 - 1))
                } else {
                    new_values.push(*n);
                }
            }
            values = new_values;
        }
        assert_eq!(values.len(), 1);
        values.first().unwrap().0
    }

    fn add(&mut self, mut o: Number) {
        self.n.append(&mut o.n);
        self.n.iter_mut().for_each(|t| t.1 += 1);
    }
}

fn parse_pair(chars_iter: &mut Chars, depth: usize) -> Number {
    match chars_iter.next() {
        Some('[') => {
            let mut n1 = parse_pair(chars_iter, depth + 1);
            match chars_iter.next() {
                Some(',') => (),
                _ => panic!("unparsable 1"),
            }
            let mut n2 = parse_pair(chars_iter, depth + 1);
            match chars_iter.next() {
                Some(']') => (),
                _ => panic!("unparsable 2"),
            }
            n1.n.append(&mut n2.n);
            n1
        }
        Some(c) => Number {
            n: vec![(c.to_digit(10).unwrap() as u64, depth)],
        },
        _ => panic!("what lmao"),
    }
}

fn parse_line(l: &str) -> Number {
    parse_pair(&mut l.chars(), 0)
}

fn parse_input(input: &str) -> Vec<Number> {
    input.lines().map(parse_line).collect()
}

fn run(input: &str) -> Option<(u64, u64)> {
    let mut numbers: Vec<Number> = parse_input(input);
    let numbers_p2 = numbers.clone();
    let mut a = numbers.remove(0);
    a.reduce();

    for num in numbers {
        a.add(num);
        a.reduce();
    }

    let p1 = a.magnitude();

    let mut max_magnitude = 0;

    for n1 in numbers_p2.clone() {
        for n2 in numbers_p2.clone() {
            let mut n_calc = n1.clone();
            n_calc.add(n2);
            n_calc.reduce();
            let magnitude = n_calc.magnitude();
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }
    }

    let p2 = max_magnitude;

    Some((p1, p2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day18 p1 : {}", p1);
    println!("Day18 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    assert_eq!(3654, p1);
    assert_eq!(4578, p2);
}

#[test]
fn test_input_sample_1() {
    let (p1, p2) = run(include_str!("input_sample_1.txt")).unwrap();
    assert_eq!(3488, p1);
    assert_eq!(3946, p2);
}

#[test]
fn test_input_sample_2() {
    let (p1, p2) = run(include_str!("input_sample_2.txt")).unwrap();
    assert_eq!(4140, p1);
    assert_eq!(3993, p2);
}
