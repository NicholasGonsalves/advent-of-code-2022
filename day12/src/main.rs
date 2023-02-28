use std::cmp::min;
use std::collections::{HashMap, VecDeque, HashSet};
use std::iter::zip;

fn find_coordinates_of_character(symbol: char, grid: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return (i.try_into().unwrap(), j.try_into().unwrap());
            }
        }
    }
    panic!("Failed to find symbol/character '{}'!", symbol)
}

/// Check if a (i, j) coordinate is within the bounds of the grid
fn in_bounds(i: i32, j: i32, grid_height: i32, grid_width: i32) -> bool {
    if i < 0 || i > grid_height-1 || j < 0 || j > grid_width-1 {
        return false;
    }
    true
}

fn search(start: (i32, i32), grid: &Vec<Vec<char>>, alpha_lookup: &HashMap<char, i32>) -> usize {
    type Point = (i32, i32);
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
                            
    let mut visited = HashSet::<Point>::new();
    let mut queue = VecDeque::<Vec<Point>>::new();
    queue.push_front(vec![start]);

    while !queue.is_empty() {

        let path = queue.pop_back().unwrap();
        let curr = path.last().unwrap();

        // Check we haven't already considered this point
        if visited.contains(curr) {
            continue
        }
        visited.insert(*curr);

        // Check if we've found the 'end'
        let curr_height = grid[curr.0 as usize][curr.1 as usize];
        if curr_height == 'E' {
            return path.len() - 1;
        }
        let curr_height_value = alpha_lookup.get(&curr_height).unwrap();


        for direction in &directions {
            let neighbour = (curr.0 + direction.0, curr.1 + direction.1);

            // Check if neighbour is in-bounds
            if !in_bounds(
                neighbour.0,
                neighbour.1,
                grid.len().try_into().unwrap(),
                grid[0].len().try_into().unwrap(),
            ) {
                continue;
            }

            // Check if we can step to this neighbour (cannot >1 step 'uphill')
            let neighbour_height = grid[neighbour.0 as usize][neighbour.1 as usize];
            let neighbour_height_value = alpha_lookup.get(&neighbour_height).unwrap();

            if (neighbour_height_value - curr_height_value) > 1 {
                continue;
            }

            let mut path_copy = path.clone();
            path_copy.push(neighbour);
            queue.push_front(path_copy);
        }

    }

    panic!("We didn't get to the end!!")

}

fn main() {
    // Parse heighmap
    let heightmap: Vec<Vec<char>> = include_str!("day12.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    
    // Construct priority value lookup hashmap
    let alphabet = "abcdefghijklmnopqrstuvwxyzES".chars();
    let mut alphabet_index: Vec<i32> = (0..26).collect();
    alphabet_index.push(25);
    alphabet_index.push(0);
    let alpha_lookup: HashMap<char, i32> = HashMap::from_iter(zip(alphabet, alphabet_index));

    // Find start coordinates
    let start_coords = find_coordinates_of_character('S', &heightmap);

    // Compute shortest path from 'S' to 'E'
    // Dijkstra was preferred but I swapped to BFS to fix an unrelated bug (classic) and I won't go and revert it now
    let distance = search(start_coords, &heightmap, &alpha_lookup);

    println!("Part 1 (shortest path): {}", distance);

    // Brute force, lets see if rust's speed can make up for less thinking (on this input size anyway..)
    let mut shortest_distance = i32::MAX;
    for i in 0..heightmap.len() {
        for j in 0..heightmap[0].len() {
            let height = heightmap[i][j];
            if *alpha_lookup.get(&height).unwrap() != 1 {
                continue
            }
            shortest_distance = min(shortest_distance, search((i.try_into().unwrap(), j.try_into().unwrap()), &heightmap, &alpha_lookup).try_into().unwrap());
        }
    }

    println!("Part 2 (shortest path): {}", shortest_distance+1);
    
}
