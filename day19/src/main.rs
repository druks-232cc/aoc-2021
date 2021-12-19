use std::{ops::Sub, collections::HashSet};

#[derive(Debug, Clone, PartialEq, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Sub for Point {
    // Not really a point, but whatever, you get the point, haha
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point {
    fn rotate(&mut self, n: u32) {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        let r = match n % 24 {
            0 => [x, y, z],
            1 => [-x, -y, z],
            2 => [-x, -z, -y],
            3 => [x, -y, -z],
            4 => [y, x, -z],
            5 => [-x, y, -z],
            6 => [-y, -x, -z],
            7 => [-x, z, y],
            8 => [z, -y, x],
            9 => [-z, -y, -x],
            10 => [y, -x, z],
            11 => [-y, x, z],
            12 => [x, z, -y],
            13 => [x, -z, y],
            14 => [y, -z, -x],
            15 => [-z, x, -y],
            16 => [-y, -z, x],
            17 => [z, -x, -y],
            18 => [y, z, x],
            19 => [z, x, y],
            20 => [-y, z, -x],
            21 => [-z, -x, y],
            22 => [z, y, -x],
            23 => [-z, y, x],
            _ => panic!("already died inside doing this, and it doesn't work!?!"),
        };

        self.x = r[0];
        self.y = r[1];
        self.z = r[2];
    }

    fn distance(p1: Point, p2: Point) -> isize {
        (p2.x - p1.x).pow(2) + (p2.y - p1.y).pow(2) + (p2.z - p1.z).pow(2)
    }

    fn match_tuples(t1: (Point, Point), t2: (Point, Point)) -> Option<(u32, isize, isize, isize)> {
        for i in 0..24 {
            let mut rot1 = t2.0.clone();
            let mut rot2 = t2.1.clone();

            rot1.rotate(i);
            rot2.rotate(i);

            if (rot2 - rot1) == (t1.1 - t1.0) {
                return Some((i, t1.1.x - rot2.x, t1.1.y - rot2.y, t1.1.z - rot2.z));
            } else if (rot2 - rot1) == (t1.0 - t1.1) {
                return Some((i, t1.0.x - rot2.x, t1.0.y - rot2.y, t1.0.z - rot2.z));
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Point>,
    distances: Vec<isize>,
    distances_hashset: HashSet<isize>,
    index: u32,
    orientation: u32,
    subscanners: Vec<(u32, (isize, isize, isize))>,
    shift: (isize, isize, isize),
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            beacons: vec![],
            distances: vec![],
            distances_hashset: HashSet::new(),
            index: 0,
            orientation: 0,
            subscanners: vec![],
            shift: (0, 0, 0),
        }
    }

    fn from(s: Scanner) -> Scanner {
        let mut m = Scanner::new();
        m.add(s);
        m
    }

    fn add(&mut self, s: Scanner) {
        for b in s.beacons.clone() {
            if !self.beacons.contains(&b) {
                self.beacons.push(b)
            }
        }
        self.subscanners.push((s.index, s.shift));
        self.calculate_distances();
    }

    fn calculate_distances(&mut self) {
        self.distances = vec![];
        for (i, p1) in self.beacons.clone().iter().enumerate() {
            for (j, p2) in self.beacons.clone().iter().enumerate() {
                if j > i {
                    let distance = Point::distance(p1.clone(), p2.clone());
                    self.distances.push(distance);
                    self.distances_hashset.insert(distance);
                }
            }
        }
    }

    fn shift(&mut self, delta_x: isize, delta_y: isize, delta_z: isize) {
        self.shift.0 += delta_x;
        self.shift.1 += delta_y;
        self.shift.2 += delta_z;
        for b in self.beacons.iter_mut() {
            b.x += delta_x;
            b.y += delta_y;
            b.z += delta_z;
        }
    }

    fn rotate(&mut self, n: u32) {
        // rotate back to 0, each rotation has a counterpart
        // some are identity if done twice
        let r = match self.orientation {
            0..=9 => self.orientation,
            10 => 11,
            11 => 10,
            12 => 13,
            13 => 12,
            14 => 15,
            15 => 14,
            16 => 17,
            17 => 16,
            18 => 19,
            19 => 18,
            20 => 21,
            21 => 20,
            22 => 23,
            23 => 22,
            _ => 0,
        };
        self.beacons.iter_mut().for_each(|b| b.rotate(r));

        self.beacons.iter_mut().for_each(|b| b.rotate(n));
        self.orientation = n;
    }

    fn get_points_for_index(&self, index: usize) -> (Point, Point) {
        // dumb but... well...
        let mut k = 0;
        for (i, p1) in self.beacons.clone().iter().enumerate() {
            for (j, p2) in self.beacons.clone().iter().enumerate() {
                if j > i {
                    if k == index {
                        return (p1.clone(), p2.clone());
                    }
                    k += 1;
                }
            }
        }
        panic!("unreachable with index {}", index);
    }

    fn overlaping_probes(&self, s: Scanner) -> (u32, Vec<(usize, usize)>) {
        let mut overlaping_probes = 0;
        let mut matching_indexes = vec![];

        for (i, d) in s.distances.iter().enumerate() {
            if self.distances_hashset.contains(&d) {
                matching_indexes.push((self.distances.iter().position(|v| v == d).unwrap(), i));
                overlaping_probes += 1;
            }
        }

        (overlaping_probes, matching_indexes)
    }
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut scanner_list = vec![];
    let mut current_scanner: Scanner = Scanner::new();
    let mut index = 0;

    for line in input.lines() {
        if line.starts_with("---") {
            if current_scanner.beacons.len() > 0 {
                scanner_list.push(current_scanner);
            }
            current_scanner = Scanner::new();
            current_scanner.index = index;
            index += 1;
        } else if line.len() == 0 {
            continue;
        } else {
            let mut coordinates = line.split(',');
            current_scanner.beacons.push(Point {
                x: coordinates.next().unwrap().parse().unwrap(),
                y: coordinates.next().unwrap().parse().unwrap(),
                z: coordinates.next().unwrap().parse().unwrap(),
            })
        }
    }

    scanner_list.push(current_scanner);
    scanner_list
}

fn run(input: &str) -> Option<(usize, isize)> {
    let mut scanners: Vec<Scanner> = parse_input(input);

    for s in scanners.iter_mut() {
        s.calculate_distances();
    }

    let mut main = Scanner::from(scanners.remove(0));

    while scanners.len() > 0 {
        for (i, s) in scanners.clone().iter_mut().enumerate() {
            let (o, indexes) = main.overlaping_probes(s.clone());
            if o > 12 {
                let mut pmatch = None;
                for index in indexes.clone() {
                    let (p1, p2) = main.get_points_for_index(index.0);
                    let (p3, p4) = s.get_points_for_index(index.1);

                    pmatch = Point::match_tuples((p1, p2), (p3, p4));
                    if pmatch.is_some() {
                        break;
                    }
                }
                let rotation = pmatch.unwrap().0;
                let delta_x = pmatch.unwrap().1;
                let delta_y = pmatch.unwrap().2;
                let delta_z = pmatch.unwrap().3;
                s.rotate(rotation);
                s.shift(delta_x, delta_y, delta_z);
                main.add(s.clone());
                scanners.remove(i);
                break;
            }
        }
    }

    let p1 = main.beacons.len();

    let mut largest_distance: isize = 0;

    for (i, s1) in main.subscanners.iter().enumerate() {
        for (j, s2) in main.subscanners.iter().enumerate() {
            if j > i {
                let d = ((s1.1).0 - (s2.1).0).abs()
                    + ((s1.1).1 - (s2.1).1).abs()
                    + ((s1.1).2 - (s2.1).2).abs();
                if d > largest_distance {
                    largest_distance = d;
                }
            }
        }
    }

    let p2 = largest_distance;

    Some((p1, p2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day19 p1 : {}", p1);
    println!("Day19 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    assert_eq!(483, p1);
    assert_eq!(14804, p2);
}

#[test]
fn test_input_sample_1() {
    let (p1, p2) = run(include_str!("input_sample_1.txt")).unwrap();
    assert_eq!(79, p1);
    assert_eq!(3621, p2);
}
