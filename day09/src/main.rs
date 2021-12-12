const MAP_SIZE: usize = 100;

type HeightMap = [[u32; MAP_SIZE]; MAP_SIZE];
type BassinMap = [[bool; MAP_SIZE]; MAP_SIZE];

trait T {
    fn is_lowpoint(&self, i: usize, j: usize) -> bool;
}

impl T for HeightMap {
    fn is_lowpoint(&self, i: usize, j: usize) -> bool {
        let p = self[i][j];
        let surroundings = [(-1,0), (0,-1), (1,0), (0,1)];

        match p {
            9 => false,
            0 => true,
            _ => surroundings.iter().fold(true, |is_low, s|
                {
                    let x = i as isize + s.0;
                    let y = j as isize + s.1;

                    if x < 0 || x >= MAP_SIZE as isize || y < 0 || y >= MAP_SIZE as isize {
                        is_low && true
                    } else {
                        is_low && ( p < self[x as usize][y as usize] )
                    }
                }
            )
        }
    }
}

struct Map {
    h: HeightMap,
    b: BassinMap,
}

impl Map {
    fn new() -> Map {
        Map { h: [[0; MAP_SIZE]; MAP_SIZE], b: [[false; MAP_SIZE]; MAP_SIZE]}
    }

    fn get_bassin_size(&mut self, i: usize, j: usize) -> u32 {
        let p = self.h[i][j];
        let surroundings = [(-1,0), (0,-1), (1,0), (0,1)];


        if p == 9 || self.b[i][j] {
            return 0
        }

        self.b[i][j] = true;


        return surroundings.iter().fold(1, |bs, s|
            {
                let x = i as isize + s.0;
                let y = j as isize + s.1;

                if x < 0 || x >= MAP_SIZE as isize || y < 0 || y >= MAP_SIZE as isize {
                    return bs
                }
                if p < self.h[x as usize][y as usize] {
                    return bs + self.get_bassin_size(x as usize, y as usize)
                } else {
                    bs
                }
            }
        );
    }
}

fn parse_line(l: &str) -> [u32; MAP_SIZE] {
    l.chars().map(|c| c.to_digit(10).unwrap())
     .collect::<Vec<u32>>().try_into().unwrap()
}

fn run(input: &str) -> Option<()> {
    let mut m = Map::new();

    m.h = input.lines()
               .map(parse_line)
               .collect::<Vec<[u32; MAP_SIZE]>>()
               .try_into().unwrap();

    let mut lowpoint_risk_value = 0;
    let mut bassin_sizes: Vec<u32> = vec![];

    for i in 0..MAP_SIZE {
        for j in 0..MAP_SIZE {
            if m.h.is_lowpoint(i,j) {
                lowpoint_risk_value += m.h[i][j] + 1;
                bassin_sizes.push(m.get_bassin_size(i,j));
            }
        }
    }

    println!("Day9 p1 : {}", lowpoint_risk_value);

    bassin_sizes.sort();

    println!("Day9 p2 : {}", bassin_sizes.pop()?*bassin_sizes.pop()?*bassin_sizes.pop()?);

    Some(())
}

fn main() {
    run(include_str!("input.txt"));
}