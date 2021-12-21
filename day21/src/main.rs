use cached::proc_macro::cached;

struct Dice {
    value: u64,
    rolls: u64,
}

impl Dice {
    fn new() -> Dice {
        Dice { value: 1, rolls: 0 }
    }

    fn next(&mut self) -> u64 {
        let v = self.value;
        self.value = (self.value % 100) + 1;
        v
    }

    fn roll(&mut self, n: u64) -> u64 {
        let mut r = 0;
        self.rolls += n;
        for _i in 0..n {
            r += self.next();
        }
        r
    }
}

fn parse_input(input: &str) -> (u64, u64) {
    let mut iter = input.lines().take(2);
    let p1 = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let p2 = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    (p1, p2)
}

fn solve_p1(s_pos_p1: u64, s_pos_p2: u64) -> u64 {
    let mut d = Dice::new();
    let mut pos_p1 = s_pos_p1;
    let mut pos_p2 = s_pos_p2;
    let mut score_p1 = 0;
    let mut score_p2 = 0;
    let mut turn = 0;

    while score_p1 < 1000 && score_p2 < 1000 {
        if turn % 2 == 0 {
            pos_p1 = ((pos_p1 + d.roll(3) - 1) % 10) + 1;
            score_p1 += pos_p1;
        } else {
            pos_p2 = ((pos_p2 + d.roll(3) - 1) % 10) + 1;
            score_p2 += pos_p2;
        }
        turn += 1;
    }

    if turn % 2 == 0 {
        d.rolls * score_p1
    } else {
        d.rolls * score_p2
    }
}

#[cached]
fn solve_p2(pos_p1: u64, pos_p2: u64, score_p1: u64, score_p2: u64) -> (u64, u64) {
    if score_p1 >= 21 {
        return (1, 0);
    } else if score_p2 >= 21 {
        return (0, 1);
    }
    let d = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    let mut p1_win = 0;
    let mut p2_win = 0;

    for roll in d {
        let new_pos = ((pos_p1 + roll.0 - 1) % 10) + 1;
        let (p2, p1) = solve_p2(pos_p2, new_pos, score_p2, score_p1 + new_pos);
        p1_win += p1 * roll.1;
        p2_win += p2 * roll.1;
    }

    (p1_win, p2_win)
}

fn run(input: &str) -> Option<(u64, u64)> {
    let starting_pos = parse_input(input);

    let p1 = solve_p1(starting_pos.0, starting_pos.1);

    let res = solve_p2(starting_pos.0, starting_pos.1, 0, 0);
    let p2 = res.0.max(res.1);

    Some((p1, p2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day21 p1 : {}", p1);
    println!("Day21 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    assert_eq!(720750, p1);
    assert_eq!(275067741811212, p2);
}

#[test]
fn test_input_sample_1() {
    let (p1, p2) = run("Player 1 starting position: 4\nPlayer 2 starting position: 8\n").unwrap();
    assert_eq!(739785, p1);
    assert_eq!(444356092776315, p2);
}
