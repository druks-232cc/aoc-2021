#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cube {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl Cube {
    fn from(c: [(isize, isize); 3]) -> Cube {
        Cube {
            x: c[0],
            y: c[1],
            z: c[2],
        }
    }

    // axis overlap binary mapped over u8
    // 0b00000zzz0yyy0xxx
    fn overlaps(self, c: Cube) -> u16 {
        let mut r = 0;
        r += range_overlaps(self.x, c.x);
        r += range_overlaps(self.y, c.y) << 4;
        r += range_overlaps(self.z, c.z) << 8;
        r
    }

    fn inside(self, c: Cube) -> bool {
        self.overlaps(c) ^ 0b001100110011 == 0
    }

    // return "on" cubes
    // c is supposed on
    // s is wether self is on or not
    fn split(self, c: Cube) -> Option<Vec<Cube>> {
        let mut subcubes = vec![];
        let mut new_c = self.clone();
        let o = c.overlaps(self);

        // inside
        if o ^ 0b001100110011 == 0 {
            panic!("hmmmm");
            // if s {
            //     // inside already on cube, so no new cubes
            //     return Some(vec![c])
            // }
        }
        if o & 0xF00 > 0 && o & 0x0F0 > 0 && o & 0x00F > 0 {
            if new_c.x.0 < c.x.0 {
                subcubes.push(Cube {
                    x: (new_c.x.0, c.x.0 - 1),
                    y: (new_c.y.0, new_c.y.1),
                    z: (new_c.z.0, new_c.z.1),
                });
                new_c.x.0 = c.x.0;
            }
            if new_c.x.1 > c.x.1 {
                subcubes.push(Cube {
                    x: (c.x.1 + 1, new_c.x.1),
                    y: (new_c.y.0, new_c.y.1),
                    z: (new_c.z.0, new_c.z.1),
                });
                new_c.x.1 = c.x.1;
            }
            if new_c.y.0 < c.y.0 {
                subcubes.push(Cube {
                    x: (new_c.x.0, new_c.x.1),
                    y: (new_c.y.0, c.y.0 - 1),
                    z: (new_c.z.0, new_c.z.1),
                });
                new_c.y.0 = c.y.0;
            }
            if new_c.y.1 > c.y.1 {
                subcubes.push(Cube {
                    x: (new_c.x.0, new_c.x.1),
                    y: (c.y.1 + 1, new_c.y.1),
                    z: (new_c.z.0, new_c.z.1),
                });
                new_c.y.1 = c.y.1;
            }
            if new_c.z.0 < c.z.0 {
                subcubes.push(Cube {
                    x: (new_c.x.0, new_c.x.1),
                    y: (new_c.y.0, new_c.y.1),
                    z: (new_c.z.0, c.z.0 - 1),
                });
                new_c.z.0 = c.z.0;
            }
            if new_c.z.1 > c.z.1 {
                subcubes.push(Cube {
                    x: (new_c.x.0, new_c.x.1),
                    y: (new_c.y.0, new_c.y.1),
                    z: (c.z.1 + 1, new_c.z.1),
                });
                new_c.z.1 = c.z.1;
            }
            return Some(subcubes);
        }
        subcubes.push(new_c);
        Some(subcubes)
    }
}

// 0: no overlaps,  000
// 1: r1.0 overlap, 001
// 2: r1.1 overlap  010
// 3: both overlap (r1 inside r2) 011
// 4: r2 inside r1  100
fn range_overlaps(r1: (isize, isize), r2: (isize, isize)) -> u16 {
    let mut count = 0;
    if r2.0 <= r1.0 && r1.0 <= r2.1 {
        count += 1;
    }
    if r2.0 <= r1.1 && r1.1 <= r2.1 {
        count = (count << 1) + 1;
    }
    if r1.0 < r2.0 && r2.1 < r1.1 {
        count = 4
    }
    count
}

fn count_lit(cubes: Vec<Cube>) -> u64 {
    let mut count = 0;
    for c in cubes {
        count += ((c.x.1 - c.x.0 + 1) * (c.y.1 - c.y.0 + 1) * (c.z.1 - c.z.0 + 1)) as u64
    }
    count
}

fn process(ins: Vec<(Cube, bool)>, limit: isize) -> Vec<Cube> {
    let mut cubes = vec![];
    let boundary = Cube::from([(-limit, limit), (-limit, limit), (-limit, limit)]);
    let mut ins = ins.clone();

    cubes.push(ins.remove(0).0);

    for (i, s) in ins {
        if !i.inside(boundary) {
            continue;
        }
        let mut new_cubes = vec![];
        for c in cubes {
            let tmp = c.split(i);
            if tmp.is_some() {
                new_cubes.append(&mut tmp.unwrap());
            }
        }
        if s {
            new_cubes.push(i);
        }
        cubes = new_cubes;
    }

    cubes
}

fn parse_input(input: &str) -> Vec<(Cube, bool)> {
    let mut list = vec![];

    for l in input.lines() {
        let (status, coordinates_str) = l.split_once(" ").unwrap();
        let status = if status == "on" { true } else { false };
        let mut coordinates = [(0, 0), (0, 0), (0, 0)];

        for (i, range) in coordinates_str.split(",").enumerate() {
            let (a, b) = range.split_once("=").unwrap().1.split_once("..").unwrap();
            coordinates[i] = (a.parse().unwrap(), b.parse().unwrap());
        }

        list.push((Cube::from(coordinates), status));
    }

    list
}

fn run(input: &str) -> Option<(u64, u64)> {
    let instructions: Vec<(Cube, bool)> = parse_input(input);

    let cubes = process(instructions.clone(), 50);

    let p1 = count_lit(cubes.clone());

    let cubes = process(instructions, 10000000);

    let p2 = count_lit(cubes.clone());

    Some((p1, p2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day22 p1 : {}", p1);
    println!("Day22 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    assert_eq!(615700, p1);
    assert_eq!(1236463892941356, p2);
}

#[test]
fn test_input_sample_1() {
    let (p1, p2) = run(include_str!("input_sample_1.txt")).unwrap();
    assert_eq!(590784, p1);
    assert_eq!(39769202357779, p2);
}

#[test]
fn test_input_sample_2() {
    let (p1, p2) = run(include_str!("input_sample_2.txt")).unwrap();
    assert_eq!(474140, p1);
    assert_eq!(2758514936282235, p2);
}
