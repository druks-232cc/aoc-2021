use std::cmp::{min,max};

const MAP_SIZE: usize = 1000;

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

struct Map {
    map: [[u32; MAP_SIZE]; MAP_SIZE],
}

impl Map {
    fn new() -> Map {
        Map { map: [[0; MAP_SIZE]; MAP_SIZE] }
    }

    fn draw_straight_line(&mut self, a: Point, b: Point) {
        if a.x == b.x {
            let start = min(a.y, b.y);
            let end = max(a.y, b.y);
            self.map[a.x][start..end+1].iter_mut().for_each(|x| *x+=1);
        } else if a.y == b.y {
            let start = min(a.x, b.x);
            let end = max(a.x, b.x);
            self.map[start..end+1].iter_mut().for_each(|s| s[a.y] += 1);
        }
    }

    fn draw_diagonal_line(&mut self, a: Point, b: Point) {
        let x_dist = max(a.x, b.x) - min(a.x, b.x);
        let y_dist = max(a.y, b.y) - min(a.y, b.y);

        if x_dist == y_dist {
            let x_dir: isize = (b.x as isize - a.x as isize) / x_dist as isize;
            let y_dir: isize = (b.y as isize - a.y as isize) / y_dist as isize;
            for i in 0..(x_dist+1) as isize {
                let j: usize = (a.x as isize + x_dir*i).try_into().unwrap();
                let k: usize = (a.y as isize + y_dir*i).try_into().unwrap();
                self.map[j][k] += 1;
            }
        }
    }

    fn dangers(&self) -> usize {
        self.map.iter()
                .flat_map(|x| x.iter())
                .filter(|x| x >= &&2)
                .count()
    }
}

fn parse_points(line: &str) -> (Point, Point) {
    let mut l = line.split_ascii_whitespace();
    let a = l.next().unwrap().split_once(',').unwrap();
    l.next();
    let b = l.next().unwrap().split_once(',').unwrap();
    (
        Point {
            x: a.0.parse().unwrap(),
            y: a.1.parse().unwrap(),
        },
        Point {
            x: b.0.parse().unwrap(),
            y: b.1.parse().unwrap(),
        },
    )
}

fn run(input: &str) -> Option<()> {
    let lines: Vec<(Point, Point)> = input.lines()
                                          .map(parse_points)
                                          .collect();

    let mut m = Map::new();

    for l in &lines {
        m.draw_straight_line(l.0, l.1);
    }

    println!("Day5 p1 : {}", m.dangers());

    for l in lines {
        m.draw_diagonal_line(l.0, l.1);
    }

    println!("Day5 p2 : {}", m.dangers());

    Some(())
}

fn main() {
    run(include_str!("input.txt"));
}