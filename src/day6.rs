use std::collections::HashMap;

fn wait_for_a_day(fishes: &mut HashMap<u32, u64>) {
    let to_die = *fishes.entry(0).or_insert(0);
    *fishes.entry(7).or_insert(0) += to_die;
    for age in 0..8 {
        *fishes.entry(age).or_insert(0) = *fishes.entry(age+1).or_insert(0);
    }
    *fishes.entry(8).or_insert(0) = to_die;
}

pub(crate) fn run(input: &str) -> Option<()> {
    let fishes: Vec<u32> = input.lines().next()?
                                    .split(',')
                                    .map(|x| x.parse().unwrap())
                                    .collect();

    let mut fishes_map: HashMap<u32, u64> = HashMap::new();

    for fish in fishes {
        *fishes_map.entry(fish).or_insert(0) += 1;
    }

    for _day in 0..80 {
        wait_for_a_day(&mut fishes_map);
    }

    println!("Day6 p1 : {}", fishes_map.values().sum::<u64>());

    for _day in 80..256 {
        wait_for_a_day(&mut fishes_map);
    }

    println!("Day6 p2 : {}", fishes_map.values().sum::<u64>());

    Some(())
}