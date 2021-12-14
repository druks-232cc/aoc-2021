use std::collections::HashMap;

struct Polymer {
    elements_count: HashMap<char,u64>,
    p: HashMap<char,HashMap<char,u64>>,
}

impl Polymer {
    fn new() -> Polymer {
        Polymer { elements_count: HashMap::new(), p: HashMap::new() }
    }

    fn from(starting_p: &str) -> Polymer {
        let mut p = Polymer::new();
        let mut chars = starting_p.chars();

        let mut prev_char = chars.next().unwrap();
        *p.elements_count.entry(prev_char).or_insert(0) += 1;
        for c in chars {
            *p.elements_count.entry(c).or_insert(0) += 1;
            let target = p.p.entry(prev_char).or_insert(HashMap::from([(c,0)]));
            *target.entry(c).or_insert(0) +=1;
            prev_char = c;
        }
        p
    }

    fn step_once(&mut self, rules: &HashMap<&str, char>) {
        for (c1, v1) in self.p.clone() {
            for (c2, v2) in v1 {
                let pattern: &str = &[c1,c2].iter().collect::<String>();
                let c = *rules.get(pattern).unwrap();

                *self.elements_count.entry(c).or_insert(0) += v2;
                // Break polymer
                self.p.entry(c1).and_modify(|e| *e.get_mut(&c2).unwrap() -= v2);
                // Connect to part 1
                self.p.entry(c1).and_modify(|e| *e.entry(c).or_insert(0) += v2);
                // Connect to part 2
                self.p.entry(c).and_modify(|e| *e.entry(c2).or_insert(0) += v2)
                    .or_insert(HashMap::from([(c2,v2)]));
            }
        }
    }

    fn solve(&mut self, n: u32, rules: &HashMap<&str, char>) -> u64 {
        for _i in 0..n { self.step_once(&rules); }
        let mut e_count: Vec<u64> = self.elements_count.values().cloned().collect();

        e_count.sort();
        e_count.last().unwrap()-e_count.first().unwrap()
    }
}

fn run(input: &str) -> Option<(u64,u64)> {
    let mut input_lines = input.lines();
    let mut polymer = Polymer::from(input_lines.next()?);
    let mut rules: HashMap<&str, char> = HashMap::new();

    input_lines.for_each(|l| {
        match l.split_once(" -> ") {
            Some((a,b)) => assert!(rules.insert(a, b.chars().next().unwrap()).is_none()),
            _ => (),
        }
    });

    let p1 = polymer.solve(10, &rules);

    let p2 = polymer.solve(30, &rules);

    Some((p1, p2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day14 p1 : {}", p1);
    println!("Day14 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    assert_eq!(2_975, p1);
    assert_eq!(3_015_383_850_689, p2);
}

#[test]
fn test_input_sample_1() {
    let (p1, p2) = run(include_str!("input_sample_1.txt")).unwrap();
    assert_eq!(1588, p1);
    assert_eq!(2_188_189_693_529, p2);
}