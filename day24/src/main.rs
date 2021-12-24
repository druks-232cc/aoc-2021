fn process_block(w: i64, z: i64, b: i64, c: i64) -> i64 {
    if b >= 10 {
        // happens 7 times
        z * 26 + w + c
    } else if w == (b + (z % 26)) {
        // should happen 7 times
        z / 26
    } else {
        // should avoid
        z + w + c
    }
    //  z = z/a * (1 + 25 * (w != (b + (z % 26)))) + (w + c) * (w != (b + (z % 26)))
    //  z/a + (w + c + 25*(z/a)) * (w != (b + (z % 26))) as i64

    // 1 =< w + c =< 23
    // n5 : n4 - 6
    // n8 : n7
    // n10 : n9 + 7
    // n11 : n6 - 8
    // n12 : n3 + 1
    // n13 : n2 - 3
    // n14 : n1 - 2
}

fn run() -> Option<(u64, u64)> {
    // converted input manually
    let _a = [1, 1, 1, 1, 26, 1, 1, 26, 1, 26, 26, 26, 26, 26];
    let b = [12, 11, 10, 10, -16, 14, 12, -4, 15, -7, -8, -4, -15, -8];
    let c = [6, 12, 5, 10, 7, 0, 4, 12, 14, 13, 10, 11, 9, 9];

    // based on deduced number relations
    let monad_max = [9, 9, 8, 9, 3, 9, 9, 9, 2, 9, 1, 9, 6, 7];
    let monad_min = [3, 4, 1, 7, 1, 9, 1, 1, 1, 8, 1, 2, 1, 1];

    for monad in [monad_max, monad_min] {
        let mut z = 0;
        for (i, index) in monad.iter().zip(0..14) {
            z = process_block(*i, z, b[index], c[index]);
        }
        assert_eq!(z, 0);
    }

    let p1 = monad_max
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse()
        .unwrap();

    let p2 = monad_min
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse()
        .unwrap();

    Some((p1, p2))
}

fn main() {
    let (p1, p2) = run().unwrap();
    println!("Day24 p1 : {}", p1);
    println!("Day24 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run().unwrap();
    assert_eq!(99893999291967, p1);
    assert_eq!(34171911181211, p2);
}
