
/// Takes in a &str of the form "13-53" and returns a tuple of the integers e.g. (13, 53)
fn parse_pair_to_numeric(pair: &str) -> (u32, u32) {
    let parsed: Vec<u32> = pair
        .split("-")
        .into_iter()
        .map(|v| v.parse::<u32>().unwrap())
        .collect();
    
    (*parsed.first().unwrap(), *parsed.last().unwrap())
}

/// Returns true if the range ('l' or 'r') completely contains the other range
fn complete_overlap(l1: u32, l2: u32, r1: u32, r2: u32) -> bool {
    if (l1 <= r1 && r2 <= l2) || (r1 <= l1 && l2 <= r2) {
        return true;
    }
    false
}

/// Returns true if there is a partial overlap of the 'l' and 'r' ranges
fn partial_overlap(l1: u32, l2: u32, r1: u32, r2: u32) -> bool {
    if (l1 <= r1 && r1 <= l2) || (l1 <= r2 && r2 <= l2) {
        return true;
    } else if (r1 <= l1 && l1 <= r2) || (r1 <= l2 && l2 <= r2) {
        return true;
    }
    false
}


fn main() {
    
    // Part 1: Find fully overlapping pairs
    let fully_overlapping_pairs: u32 = include_str!("day4.txt")
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();
            let (l1, l2) = parse_pair_to_numeric(left);
            let (r1, r2) = parse_pair_to_numeric(right);

            complete_overlap(l1, l2, r1, r2)  // returns bool, true if complete overlap
        })
        .map(|overlap| if overlap {1} else {0})
        .sum();

    println!("Part 1: {}", fully_overlapping_pairs);

    // Part 2: Find partially overlapping pairs
    let partially_overlapping_pairs: u32 = include_str!("day4.txt")
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();
            let (l1, l2) = parse_pair_to_numeric(left);
            let (r1, r2) = parse_pair_to_numeric(right);

            partial_overlap(l1, l2, r1, r2)  // returns bool, true if complete overlap
        })
        .map(|overlap| if overlap {1} else {0})
        .sum();

    println!("Part 2: {}", partially_overlapping_pairs);
}
