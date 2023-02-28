// Day 8: Treehouse

use std::{cmp::max, collections::HashSet};

/// Scan through the tree horizontally, updating the seen HashSet with 
/// externally visible trees.
/// A tree is visible if all of the other trees between it and an edge 
/// of the grid are shorter than it.
fn horizontal_scan(
    vertical_range: Vec<usize>,
    horizontal_range: Vec<usize>,
    forrest: &Vec<Vec<i32>>,
    seen: &mut HashSet<(usize, usize)>,
) {
    for i in vertical_range.clone() {
        let mut prev_max_tree_height: i32 = -1;
        for j in horizontal_range.clone() {
            let tree_height = forrest[i][j];
            if tree_height > prev_max_tree_height {
                seen.insert((i, j));
            }
            prev_max_tree_height = max(tree_height, prev_max_tree_height);
        }
    }
}

/// Scan through the tree vertically, updating the seen HashSet with 
/// externally visible trees.
/// A tree is visible if all of the other trees between it and an edge 
/// of the grid are shorter than it.
fn vertical_scan(
    vertical_range: Vec<usize>,
    horizontal_range: Vec<usize>,
    forrest: &Vec<Vec<i32>>,
    seen: &mut HashSet<(usize, usize)>,
) {
    for i in vertical_range.clone() {
        let mut prev_max_tree_height: i32 = -1;
        for j in horizontal_range.clone() {
            let tree_height = forrest[j][i];
            if tree_height > prev_max_tree_height {
                seen.insert((j, i));
            }
            prev_max_tree_height = max(tree_height, prev_max_tree_height);
        }
    }
}

/// Return true if point is on the edge of the forrest
fn point_on_edge(i: usize, j: usize, height: usize, width: usize) -> bool {
    if i == 0 || j == 0 || i == height-1 || j == width-1 {
        return true
    }
    false
}


fn part_1(forrest: Vec<Vec<i32>>) -> usize {
    // Approach: For each side of the forest, count the number of trees looking inwards
    //           and store their coordinates. Then count the unique visible tree coords.
    let forrest_width = forrest.len();
    let forrest_height = forrest[0].len();

    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    // Looking left->right, update unique seen trees
    let vertical_range = (0..forrest_height).collect::<Vec<usize>>();
    let horizontal_range = (0..forrest_width).collect::<Vec<usize>>();
    horizontal_scan(vertical_range, horizontal_range, &forrest, &mut seen);

    // Looking right->left, update unique seen trees
    let vertical_range = (0..forrest_height).collect::<Vec<usize>>();
    let horizontal_range = (0..forrest_width).rev().collect::<Vec<usize>>();
    horizontal_scan(vertical_range, horizontal_range, &forrest, &mut seen);

    // Looking top->bottom, update unique seen trees
    let vertical_range = (0..forrest_height).collect::<Vec<usize>>();
    let horizontal_range = (0..forrest_width).collect::<Vec<usize>>();
    vertical_scan(vertical_range, horizontal_range, &forrest, &mut seen);

    // Looking bottom->top, update unique seen trees
    let vertical_range = (0..forrest_height).collect::<Vec<usize>>();
    let horizontal_range = (0..forrest_width).rev().collect::<Vec<usize>>();
    vertical_scan(vertical_range, horizontal_range, &forrest, &mut seen);

    seen.len()
}

fn main() {

    let forrest: Vec<Vec<i32>> = include_str!("day8.txt")
        .lines()
        .map(|line| {
            let row = line
                .chars()
                .into_iter()
                .map(|tree| tree.to_digit(10).unwrap())
                .map(|tree| tree as i32)
                .collect::<Vec<i32>>();
            row
        })
        .collect();

    // Part 1: Count number of trees visible from outside the forrest.
    let visible_trees = part_1(forrest.clone());
    println!("{}", visible_trees);


    // Part 2: Figure out the scenic score 
    //         (the multiplied values of the number of trees that can be seen in 
    //         each cardinal direction).
    // Try brute force first: a better approach would be dp in 4 directions, keeping track
    // of the number of trees visible at any point in a given direction. Then we just
    // need to, for each point, use the points corresponding to each direction for each
    // tree location, from each corresponding dp grid.

    // 'Brute force'
    let mut highest_scenic_score = 0;

    let forrest_width = forrest.len();
    let forrest_height = forrest[0].len();

    for i in 0..forrest_height {
        for j in 0..forrest_width {

            // Don't bother with trees on the edge
            if point_on_edge(i, j, forrest_height, forrest_width) {
                continue
            }

            let mut scenic_score = 1;
            // Look in each direction
            let curr = forrest[i][j];
            // Look right
            let mut dist = 0;
            for k in j+1..forrest_width {
                dist += 1;
                if forrest[i][k] >= curr {
                    break
                }
            }
            scenic_score *= max(1, dist);
            // Look left
            let mut dist = 0;
            for k in (0..j).rev() {
                dist += 1;
                if forrest[i][k] >= curr {
                    break
                }
            }
            scenic_score *= max(1, dist);
            // Look up
            let mut dist = 0;
            for k in (0..i).rev() {
                dist += 1;
                if forrest[k][j] >= curr {
                    break
                }
            }
            scenic_score *= max(1, dist);
            // Look down
            let mut dist = 0;
            for k in i+1..forrest_height {
                dist += 1;
                if forrest[k][j] >= curr {
                    break
                }
            }
            scenic_score *= max(1, dist);
            // println!("Scenic score: ({},{}) {}", i, j, scenic_score);
            highest_scenic_score = max(highest_scenic_score, scenic_score);
        }
    }

    println!("{}", highest_scenic_score);

}
