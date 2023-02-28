
fn main() {

    // Part 1
    let mut calories = include_str!("day1.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>()
        .split(|line| line.is_none())
        .into_iter()
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .collect::<Vec<_>>();
    
    let max_calories = calories.iter().max().unwrap();
    println!("{max_calories:?}");

    // Part 2
    calories.sort_by_key(|&v| u64::MAX - v);

    let top_three: u64 = calories.iter().take(3).sum();
    println!("{top_three:?}");

}
