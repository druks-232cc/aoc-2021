fn fold(paper: Vec<Vec<bool>>, f: (bool, usize)) -> Vec<Vec<bool>> {
    match f.0 {
        // fold along x
        false => {
            let p1  = paper.iter()
                .map(|c| c[..f.1].to_vec())
                .collect::<Vec<Vec<bool>>>();
            let p2 = paper.iter()
                .map(|c| c[f.1+1..].to_vec())
                .collect::<Vec<Vec<bool>>>();

            p1.iter().zip(p2.iter())
                .map(|(a,b)| {
                    a.iter().zip(b.iter().rev())
                        .map(|(a,b)| a | b).collect::<Vec<bool>>()
                }).collect::<Vec<Vec<bool>>>()
        },
        // fold along y
        true => {
            let p1 = paper[..f.1].to_vec();
            let p2 = paper[f.1+1..].to_vec();

            p1.iter().zip(p2.iter().rev())
                .map(|(a,b)| {
                    a.iter().zip(b.iter())
                        .map(|(a,b)| a | b).collect::<Vec<bool>>()
                }).collect::<Vec<Vec<bool>>>()
        },
    }
}

fn run(input: &str) -> Option<(usize,usize)> {
    let mut points: Vec<(usize,usize)> = vec![];
    let mut folds: Vec<(bool,usize)> = vec![];

    input.lines().for_each(|l| {
        match l.split_once(',') {
            Some((a,b)) => points.push((a.parse().unwrap(),b.parse().unwrap())),
            _ => (),
        }
        match l.split_once('=') {
            Some((a,b)) => folds.push((a.ends_with("y"),b.parse().unwrap())),
            _ => (),
        }
    });

    let x_max = folds.iter().filter(|a| !a.0).next()?.1;
    let y_max = folds.iter().filter(|a| a.0).next()?.1;
    folds.reverse();

    // Dynamic 2D array
    let mut paper = vec![vec![false ; x_max*2+1] ; y_max*2+1];

    points.iter().for_each(|p| paper[p.1][p.0] = true);

    paper = fold(paper, folds.pop()?);

    let p1 = paper.iter().map(|l| l.iter().filter(|a| **a).count()).sum::<usize>();

    while let Some(f) = folds.pop() {
        paper = fold(paper, f);
    }

    paper.iter().for_each(|l|
        println!("{}",
            l.iter().map(|v| if *v { '#' } else {' '}).collect::<String>()
        )
    );

    let p2 = paper.iter().map(|l| l.iter().filter(|a| **a).count()).sum::<usize>();

    Some((p1, p2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day13 p1 : {}", p1);
    println!("Day13 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    assert_eq!(621, p1);
    assert_eq!(95, p2);
}

#[test]
fn test_input_sample_1() {
    let (p1, p2) = run(include_str!("input_sample_1.txt")).unwrap();
    assert_eq!(17, p1);
    assert_eq!(16, p2);
}