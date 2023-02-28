fn main() {
    // Part 1
    let score: i32 = include_str!("day2.txt")
        .lines()
        .map(|round| {
            match round {
                "A X" => 1 + 3, // draw
                "B X" => 1 + 0, // loss
                "C X" => 1 + 6, // win
                "A Y" => 2 + 6, // win
                "B Y" => 2 + 3, // draw
                "C Y" => 2 + 0, // loss
                "A Z" => 3 + 0, // loss
                "B Z" => 3 + 6, // win
                "C Z" => 3 + 3, // draw
                &_ => 0,
            }
        })
        .sum();

    println!("{score:?}");

    // Part 2
    // X = Lose, Y = Draw, Z = Win
    let score: i32 = include_str!("day2.txt")
        .lines()
        .map(|round| {
            match round {
                "A X" => 3 + 0, // lose with Z
                "B X" => 1 + 0, // lose with X
                "C X" => 2 + 0, // lose with Y
                "A Y" => 1 + 3, // draw with X
                "B Y" => 2 + 3, // draw with Y
                "C Y" => 3 + 3, // draw with Z
                "A Z" => 2 + 6, // win with Y
                "B Z" => 3 + 6, // win with Z
                "C Z" => 1 + 6, // win with X
                &_ => 0,
            }
        })
        .sum();

    println!("{score:?}");
}
