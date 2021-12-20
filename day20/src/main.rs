use std::fmt;

#[derive(Debug)]
struct Image {
    p: Vec<Vec<u8>>,
    outer: u8,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.p.iter().for_each(|l| {
            let line: String = l.iter().map(|c| match c {
                0 => ". ",
                1 => "# ",
                _ => "? ",
            }).collect();
            write!(f, "{}\n", line).expect("can't write to output");
        });
        Ok(())
    }
}

impl Image {
    fn new() -> Image {
        Image { p: vec![], outer: 0 }
    }

    fn from(input: std::str::Lines) -> Image {
        let mut i = Image::new();

        for line in input {
            i.p.push(parse_line(line));
        }

        i.extend();

        i
    }

    fn extend(&mut self) {
        self.p.iter_mut().for_each(|l| {
            l.insert(0, self.outer);
            l.push(self.outer);
        });
        self.p.insert(0, vec![self.outer; self.p[0].len()]);
        self.p.push(vec![self.outer; self.p[0].len()])
    }

    fn zoom(&self, t: &Vec<u8>, i: usize, j: usize) -> u8 {
        let cluster = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut t_index: usize = 0;

        for (k, c) in cluster.iter().enumerate() {
            let i_p = (i as isize + c.0) as usize;
            let j_p = (j as isize + c.1) as usize;
            t_index += (self.p[i_p][j_p] as usize) << (cluster.len() - 1 - k);
        }
        t[t_index]
    }

    fn enhance_once(&mut self, t: &Vec<u8>) {
        self.extend();
        // outer pixels may be flashing each times
        let mut index = 0;
        for i in 0..9 { index += (self.outer as usize) << i }
        self.outer = t[index];

        let mut new_pic = vec![];

        for (i, v) in self.p.iter().enumerate() {
            new_pic.push(vec![]);
            for (j, _c) in v.iter().enumerate() {
                // Border
                if i == 0 || i == self.p.len() - 1 || j == 0 || j == v.len() - 1 {
                    new_pic[i].push(self.outer);
                } else {
                    new_pic[i].push(self.zoom(&t, i, j));
                }
            }
        }

        self.p = new_pic;
    }

    fn enhance(&mut self, t: &Vec<u8>, n: u32) {
        for _ in 0..n {
            self.enhance_once(t);
        }
    }

    fn count_lit(&self) -> usize {
        self.p.iter().flatten().filter(|&&v| v == 1).count()
    }
}

fn parse_line(l: &str) -> Vec<u8> {
    let mut v = vec![];
    let mut chars = l.chars();

    while let Some(c) = chars.next() {
        match c {
            '.' => v.push(0),
            '#' => v.push(1),
            _ => (),
        }
    }

    v
}

fn run(input: &str) -> Option<(usize, usize)> {
    let mut input_lines = input.lines();

    let t = parse_line(input_lines.next().unwrap());
    input_lines.next();
    let mut i = Image::from(input_lines);

    // enhance twice
    i.enhance(&t, 2);

    let p1 = i.count_lit();

    // 48 more times
    i.enhance(&t, 48);

    let p2 = i.count_lit();

    Some((p1, p2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day20 p1 : {}", p1);
    println!("Day20 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    assert_eq!(5179, p1);
    assert_eq!(16112, p2);
}

#[test]
fn test_input_sample_1() {
    let (p1, p2) = run(include_str!("input_sample_1.txt")).unwrap();
    assert_eq!(35, p1);
    assert_eq!(3351, p2);
}

#[test]
fn test_input_sample_2() {
    let (p1, p2) = run(include_str!("input_sample_2.txt")).unwrap();
    assert_eq!(5326, p1);
    assert_eq!(17096, p2);
}

#[test]
fn test_input_sample_3() {
    let (p1, p2) = run(include_str!("input_sample_3.txt")).unwrap();
    assert_eq!(24, p1);
    assert_eq!(3352, p2);
}