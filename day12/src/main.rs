use std::collections::{HashMap, HashSet};

struct Caves<'a> {
    is_large: HashSet<&'a str>,
    edges: HashMap<&'a str, Vec<&'a str>>,
    paths_count: usize,
}

impl<'a> Caves<'a> {
    fn new() -> Caves<'static> {
        Caves {
            is_large: HashSet::new(),
            edges: HashMap::new(),
            paths_count: 0,
        }
    }

    fn insert_cave(&mut self, c: &'a str) {
        if c.chars().next().unwrap().is_ascii_uppercase() {
            self.is_large.insert(c);
        }
    }

    fn link(&mut self, a: &'a str, b: &'a str) {
        self.edges
            .entry(a)
            .and_modify(|e| e.append(&mut vec![b]))
            .or_insert(vec![b]);
        self.edges
            .entry(b)
            .and_modify(|e| e.append(&mut vec![a]))
            .or_insert(vec![a]);
    }

    fn map_caves(&mut self, link: (&'a str, &'a str)) {
        let (a, b) = link;
        self.insert_cave(a);
        self.insert_cave(b);
        self.link(a, b);
    }

    fn build_paths_p1(&mut self) {
        self.paths_count = 0;
        self._build_paths(false);
    }

    fn build_paths_p2(&mut self) {
        self.paths_count = 0;
        self._build_paths(true);
    }

    fn _build_paths(&mut self, visit_twice: bool) {
        let mut q: Vec<(HashSet<&str>, &str, Option<&str>)> =
            vec![(HashSet::from(["start"]), "start", None)];

        while let Some((current_path, last, small_cave)) = q.pop() {
            if last == "end" {
                self.paths_count += 1;
                continue;
            }

            let edges = self.edges.get(last).unwrap();

            for next in edges {
                let mut next_small_cave = small_cave;

                if current_path.contains(next) && !self.is_large.contains(next) {
                    if next == &"start" {
                        continue;
                    } else if visit_twice && small_cave.is_none() {
                        next_small_cave = Some(*next);
                    } else {
                        continue;
                    }
                }

                let mut next_path = current_path.clone();
                next_path.insert(next);

                q.push((next_path, next, next_small_cave));
            }
        }
    }
}

fn run(input: &str) -> Option<(usize, usize)> {
    let mut c = Caves::new();

    for l in input.lines() {
        c.map_caves(l.split_once("-").unwrap());
    }

    c.build_paths_p1();
    let p1 = c.paths_count;

    c.build_paths_p2();
    let p2 = c.paths_count;

    Some((p1, p2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day12 p1 : {}", p1);
    println!("Day12 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    assert_eq!(3497, p1);
    assert_eq!(93686, p2);
}

#[test]
fn test_input_sample_1() {
    let (p1, p2) = run(include_str!("input_sample_1.txt")).unwrap();
    assert_eq!(10, p1);
    assert_eq!(36, p2);
}

#[test]
fn test_input_sample_2() {
    let (p1, p2) = run(include_str!("input_sample_2.txt")).unwrap();
    assert_eq!(19, p1);
    assert_eq!(103, p2);
}

#[test]
fn test_input_sample_3() {
    let (p1, p2) = run(include_str!("input_sample_3.txt")).unwrap();
    assert_eq!(226, p1);
    assert_eq!(3509, p2);
}
