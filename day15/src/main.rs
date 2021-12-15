use pathfinding::prelude::dijkstra;

struct Map {
    map: Vec<usize>,
    map_size: usize,
}

impl Map {
    fn new() -> Map {
        Map { map: vec![], map_size: 0}
    }

    fn from(input: &str) -> Map {
        let mut m = Map::new();

        let input_lines = input.lines();
        m.map_size = input_lines.clone().count();

        input_lines.for_each(|l|
            l.chars().for_each(|c|
                m.map.push(c.to_digit(10).unwrap() as usize)
            )
        );

        m
    }

    fn upscale(&mut self, n: usize) {
        let mut new_map: Vec<usize> = vec![];
        let shift = self.map_size;

        // upscale horizontally
        for i in 0..shift {
            for j in 0..n {
                new_map.append(
                    &mut self.map[shift*i..shift*(i+1)]
                        .iter()
                        .map(|e| (e-1 + j) % 9 + 1)
                        .collect()
                );
            }
        }

        self.map = new_map.clone();

        // upscale vertically
        for j in 1..n {
            for i in 0..shift {
                new_map.append(
                    &mut self.map[shift*i*n..shift*(i+1)*n]
                        .iter()
                        .map(|e| (e-1 + j) % 9 + 1)
                        .collect()
                );
            }
        }

        self.map = new_map;
        self.map_size = shift*n;
    }

    fn sucessors(&self, i: &usize) -> Vec<(usize, usize)> {
        let mut s: Vec<(usize, usize)> = vec![];
        let shift = self.map_size;

        if i >= &shift {
            s.push((*i-shift,self.map[*i-shift]));
        }
        if i < &(shift*(shift-1)) {
            s.push((*i+shift,self.map[*i+shift]));
        }
        if i % shift != 0 {
            s.push((*i-1,self.map[*i-1]));
        }
        if (i+1) % shift != 0 {
            s.push((*i+1,self.map[*i+1]));
        }

        s
    }
}

fn run(input: &str) -> Option<(usize,usize)> {
    let mut m = Map::from(input);

    let result = dijkstra(
        &(0 as usize),
        |i| m.sucessors(i),
        |i| *i == m.map_size*m.map_size-1
    ).expect("No path found");

    let p1 = result.1;

    m.upscale(5);

    let result = dijkstra(
        &(0 as usize),
        |i| m.sucessors(i),
        |i| *i == m.map_size*m.map_size-1
    ).expect("No path found");

    let p2 = result.1;

    Some((p1,p2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day15 p1 : {}", p1);
    println!("Day15 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    assert_eq!(447, p1);
    assert_eq!(2825, p2);
}

#[test]
fn test_input_sample_1() {
    let (p1, p2) = run(include_str!("input_sample_1.txt")).unwrap();
    assert_eq!(40, p1);
    assert_eq!(315, p2);
}