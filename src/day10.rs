const NAV_COMPENDIUM: [(char, char, u64, u64); 4] = [
    ('(', ')', 3, 1),
    ('[', ']', 57, 2),
    ('{', '}', 1197, 3),
    ('<', '>', 25137, 4),
];

fn parse_line(s: &str) -> (u64, bool) {
    let mut stack: Vec<char> = vec![];

    for c in s.chars() {
        if NAV_COMPENDIUM.map(|t| t.0).contains(&c) {
            stack.push(c);
            continue;
        }

        let matching_tuple = NAV_COMPENDIUM.iter()
            .filter(|t| t.1.eq(&c))
            .next().unwrap();

        if matching_tuple.0.eq(stack.last().unwrap()) {
            stack.pop();
        } else {
            // corrupted line
            return (matching_tuple.2, false)
        }
    }

    // incomplete line ; process stack
    let mut score: u64 = 0;
    while let Some(c) = stack.pop() {
        let matching_tuple = NAV_COMPENDIUM.iter()
            .filter(|t| t.0.eq(&c))
            .next().unwrap();

        score *= 5;
        score += matching_tuple.3;
    }
    (score, true)
}

pub(crate) fn run(input: &str) -> Option<()> {
    let input: Vec<(u64,bool)> = input.lines()
        .map(parse_line)
        .collect();

    let score: u64 = input.iter()
        .filter(|t| !t.1)
        .map(|t| t.0)
        .sum();

    println!("Day10 p1 : {}", score);

    let mut scores: Vec<u64> = input.iter()
        .filter(|t| t.1)
        .map(|t| t.0)
        .collect();

    scores.sort();

    let middle_score: u64 = *scores.iter().nth(scores.len()/2).unwrap();

    println!("Day10 p2 : {}", middle_score);

    Some(())
}
