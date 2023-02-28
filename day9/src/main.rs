// Day 9: Rope simulation.

use std::{collections::HashSet, str::FromStr};

#[derive(Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

/// Allow parsing of the direction enum from the input strings
impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "U" => Ok(Direction::UP),
            "D" => Ok(Direction::DOWN),
            "L" => Ok(Direction::LEFT),
            "R" => Ok(Direction::RIGHT),
            _ => Err(()),
        }
    }
}


/// Implement rope simulation, keeping track of the unique positions of the final knot (tail)
fn simulate_rope_and_count_tail_positions(
    mut rope: Vec<(i32, i32)>,
    steps: Vec<(Direction, i32)>,
) -> HashSet<(i32, i32)> {
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert(rope.last().unwrap().clone());
    for (direction, distance) in steps {
        // Find vector for updating head position based on direction
        let vector = match direction {
            Direction::UP => (0, 1),
            Direction::DOWN => (0, -1),
            Direction::LEFT => (-1, 0),
            Direction::RIGHT => (1, 0),
        };

        // For each step in the given direction, update the head & tail
        for _ in 0..distance {
            // Update head position
            let prev_rope = rope.clone();
            rope[0] = (rope[0].0 + vector.0, rope[0].1 + vector.1);

            for (i, knot) in prev_rope[1..].iter().enumerate() {
                // Calculate distance of knot from preceding knot
                let x_dist = rope[i].0 - knot.0 as i32;
                let y_dist = rope[i].1 - knot.1 as i32;

                // Update knot position
                let vector = match (x_dist, y_dist) {
                    // Within a distance of 1, we don't update the knot position
                    (0, 0) => (0, 0),
                    (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
                    (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
                    // Move directly horizontally or verically if 2+ away
                    (2, 0) => (1, 0),
                    (-2, 0) => (-1, 0),
                    (0, 2) => (0, 1),
                    (0, -2) => (0, -1),
                    // Right diagonal move
                    (2, 1) => (1, 1),
                    (2, -1) => (1, -1),
                    // Left diagonal move
                    (-2, 1) => (-1, 1),
                    (-2, -1) => (-1, -1),
                    // Diagonal hop up or down
                    (1, 2) => (1, 1),
                    (-1, 2) => (-1, 1),
                    (1, -2) => (1, -1),
                    (-1, -2) => (-1, -1),
                    // Direct diagonal move
                    (-2, -2) => (-1, -1),
                    (-2, 2) => (-1, 1),
                    (2, -2) => (1, -1),
                    (2, 2) => (1, 1),
                    _ => panic!("Unhandled case! D:"),
                };
                rope[i + 1] = (knot.0 + vector.0, knot.1 + vector.1)
            }

            // Add tail position to seen tail positions
            tail_positions.insert(rope.last().unwrap().clone());
        }
    }
    tail_positions
}

fn main() {
    // Parse input commands
    let steps: Vec<(Direction, i32)> = include_str!("day9.txt")
        .lines()
        .map(|line| {
            let instruction = line.split_once(" ").unwrap();
            let direction = Direction::from_str(instruction.0).unwrap();
            let distance = instruction.1.parse::<i32>().ok().unwrap();
            (direction, distance)
        })
        .collect();

    // Part 1: Find unique positions of Tail with a rope of length 2
    // Initalise rope of length 2
    let rope: Vec<(i32, i32)> = vec![(0, 0), (0, 0)];

    // Simulate rope and count unique tail positions
    let tail_positions = simulate_rope_and_count_tail_positions(rope, steps.clone());
    println!("Part 1: {}", tail_positions.len());

    // Part 2: Find unique positions of Tail with a rope of length 10
    let rope: Vec<(i32, i32)> = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    // Simulate rope and count unique tail positions
    let tail_positions = simulate_rope_and_count_tail_positions(rope, steps.clone());
    println!("Part 2: {}", tail_positions.len());
}
