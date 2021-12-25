fn run() -> Option<(u64, u64)> {
    // # Solved with pen and paper

    // ## Input:

    // ```
    // #############
    // #...........#
    // ###B#C#C#B###
    //   #D#D#A#A#
    //   #########
    // ```

    let p1 = 18051;
    let p2 = 50245;

    Some((p1, p2))
}

fn main() {
    let (p1, p2) = run().unwrap();
    println!("Day23 p1 : {}", p1);
    println!("Day23 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run().unwrap();
    assert_eq!(18051, p1);
    assert_eq!(50245, p2);
}
