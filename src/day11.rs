const MAP_SIZE: usize = 10;

type OctoMap = [[u32; MAP_SIZE]; MAP_SIZE];
type FlashMap = [[bool; MAP_SIZE]; MAP_SIZE];

struct Map {
    o: OctoMap,
    f: FlashMap,
    iterations: u32,
    total_flashes: usize,
}

impl Map {
    fn new() -> Map {
        Map {
            o: [[0; MAP_SIZE]; MAP_SIZE],
            f: [[false; MAP_SIZE]; MAP_SIZE],
            iterations: 0,
            total_flashes: 0,
        }
    }

    fn flash(&mut self, i: usize, j: usize) {
        let surroundings = [(-1,0), (0,-1), (1,0), (0,1),
            (-1,-1), (1,-1), (-1,1), (1,1),];

        self.f[i][j] = true;

        for s in surroundings.iter() {
            let x = i as isize + s.0;
            let y = j as isize + s.1;

            if x < 0 || x >= MAP_SIZE as isize || y < 0 || y >= MAP_SIZE as isize {
                continue;
            } else {
                self.o[x as usize][y as usize] += 1;
                if self.o[x as usize][y as usize] >= 10 && self.f[x as usize][y as usize] == false {
                    self.flash(x as usize,y as usize);
                }
            }
        }
    }

    fn step_once(&mut self) -> usize {
        self.iterations += 1;

        // increment and flash octopuses
        for i in 0..MAP_SIZE {
            for j in 0..MAP_SIZE {
                self.o[i][j] += 1;
                if self.o[i][j] >= 10 && self.f[i][j] == false {
                    self.flash(i,j);
                }
            }
        }

        let flash_count = self.f.iter().map(|l| l.iter().filter(|b| **b).count()).sum();
        self.total_flashes += flash_count;

        // reset flashed octopuses
        for i in 0..MAP_SIZE {
            for j in 0..MAP_SIZE {
                self.f[i][j] = false;
                if self.o[i][j] >= 10 {
                    self.o[i][j] = 0;
                }
            }
        }

        flash_count
    }

    fn do_n_steps(&mut self, n: u32) {
        for _ in 1..=n {
            self.step_once();
        }
    }

    fn step_until_sync_flash(&mut self) {
        loop {
            if self.step_once() == MAP_SIZE*MAP_SIZE {
                break;
            }
        }
    }
}

fn parse_line(l: &str) -> [u32; MAP_SIZE] {
    l.chars().map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>().try_into().unwrap()
}

pub(crate) fn run(input: &str) -> Option<()> {
    let mut m = Map::new();

    m.o = input.lines()
        .map(parse_line)
        .collect::<Vec<[u32; MAP_SIZE]>>()
        .try_into().unwrap();

    m.do_n_steps(100);

    println!("Day11 p1 : {}", m.total_flashes);

    m.step_until_sync_flash();

    println!("Day11 p2 : {}", m.iterations);

    Some(())
}