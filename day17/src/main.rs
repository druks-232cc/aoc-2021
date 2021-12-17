#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Zone {
    a: Point,
    b: Point,
}

fn solve_p1(z: &Zone) -> isize {
    let lower_limit = z.a.y;

    if lower_limit < 0 {
        -lower_limit * (-lower_limit - 1) / 2
    } else {
        panic!("oh shit")
    }
}

fn solve_p2(z: &Zone) -> isize {
    // Aim for the head
    let mut count = (z.b.x - z.a.x + 1) * (z.b.y - z.a.y + 1);

    // Solution for (z.a.x)=(n(n-1))/2, n being minimum x to reach target due to drag
    let x_min = ((-1.0 + (8.0 * (z.a.x as f64)).sqrt()) / 2.0).ceil() as isize;
    // Maximum x to reach target in 2 steps
    let x_max = (z.b.x + 1) / 2;

    // Aim down to reach in 2 steps
    let y_min = (z.a.y + 1) / 2;
    // Trivial
    let y_max = -z.a.y;

    // Out with the math, in with the bruteforce
    for i in x_min..=x_max {
        for j in y_min..=y_max {
            let mut x = i;
            let mut y = j;
            let mut pos = Point { x: 0, y: 0 };

            while pos.x <= z.b.x && pos.y >= z.a.y {
                pos.x += x;
                pos.y += y;
                if x > 0 {
                    x -= 1
                }
                y -= 1;

                if z.a.x <= pos.x && pos.x <= z.b.x && z.a.y <= pos.y && pos.y <= z.b.y {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

fn parse_input(input: &str) -> Option<Zone> {
    let mut iter = input
        .trim()
        .split(",")
        .map(|s| s.split_once("=").unwrap().1)
        .map(|s| s.split_once("..").unwrap())
        .map(|t| (t.0.parse().unwrap(), t.1.parse().unwrap()))
        .take(2);
    let x_range: (isize, isize) = iter.next()?;
    let y_range: (isize, isize) = iter.next()?;

    Some(Zone {
        a: Point {
            x: x_range.0,
            y: y_range.0,
        },
        b: Point {
            x: x_range.1,
            y: y_range.1,
        },
    })
}

fn run(input: &str) -> Option<(isize, isize)> {
    let zone: Zone = parse_input(input)?;

    let p1 = solve_p1(&zone);

    let p2 = solve_p2(&zone);

    Some((p1, p2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day17 p1 : {}", p1);
    println!("Day17 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    assert_eq!(9180, p1);
    assert_eq!(3767, p2);
}

#[test]
fn test_input_sample_1() {
    let (p1, p2) = run("target area: x=20..30, y=-10..-5").unwrap();
    assert_eq!(45, p1);
    assert_eq!(112, p2);
}
