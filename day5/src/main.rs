use std::collections::VecDeque;

/// Transpose the 2D vec of chars
fn transpose(rows: Vec<VecDeque<char>>) -> Vec<VecDeque<char>> {
    let height = rows.len();
    let width = rows.first().unwrap().len();
    let mut transposed: Vec<VecDeque<char>> =
        vec![VecDeque::from(vec![' '; height as usize]); width as usize];
    for i in 0..height {
        for j in 0..width {
            transposed[j][i] = rows[i][j]
        }
    }
    transposed
}

/// Given a vector of crate stacks (VecDeque(s)), return the top crate for each stack in a concat string format
fn get_top_crates(stacks: Vec<VecDeque<char>>) -> String {
    let top_crates = String::from_iter(
        stacks
            .iter()
            .map(|stack| stack.front().unwrap().to_owned())
            .collect::<Vec<char>>(),
    );

    top_crates
}

fn main() {
    // Split input into cargo and instructions sections
    let (cargo, instructions) = include_str!("day5.txt").split_once("\n\n").unwrap();

    println!("{}", cargo);

    // Split cargo section of input into 2D vector of cargo labels
    let rows: Vec<VecDeque<char>> = cargo
        .lines()
        .map(|line| {
            // Parse into rows of chars
            let mut row: VecDeque<char> = VecDeque::from(vec![' '; 9 as usize]);
            let mut counter = 0;
            for (i, label) in line.chars().enumerate() {
                if i % 4 == 1 {
                    // Labels occur every 4 characters, starting at char index 1
                    row[counter] = label;
                    counter += 1;
                }
            }
            row
        })
        .collect();

    // Exclude the final row, which contains only the stack label (1 to 9)
    let crate_rows = rows.split_last().unwrap().1.to_owned();

    // Transpose the rows, so each inner vector represents a cargo stack of crate labels
    let mut stacks = transpose(crate_rows);

    // Pop chars with values ' ', as they are not required
    for stack in &mut stacks {
        while stack.front().unwrap().clone() == ' ' {
            stack.pop_front();
        }
    }
    // println!("{:#?}", stacks);

    // Save this format of stacks for part 2
    let orginal_stacks = stacks.clone();

    // Parse instructions strings into integer values
    // Instruction format: move 11 from 3 to 9
    let parsed_instructions: Vec<(usize, usize, usize)> = instructions
        .lines()
        .map(|instruction| instruction.split(" ").collect::<Vec<&str>>())
        .map(|instruction| match instruction[..] {
            // Destructure each instruction into tuple of volume, source stack, and destination stack
            [_, volume, _, source, _, destination] => (
                volume.parse::<usize>().unwrap(),
                source.parse::<usize>().unwrap(),
                destination.parse::<usize>().unwrap(),
            ),
            _ => todo!("We guarentee no other format."),
        })
        .collect::<Vec<(usize, usize, usize)>>();

    // println!("{:#?}", parsed_instructions);

    // Apply instructions to cargo stacks
    // Use cloned parsed_instructions due to applied consumption of the values (that we need for part 2)
    for (volume, source, destination) in parsed_instructions.clone() {
        let mut source_stack = stacks.get_mut(source - 1).unwrap().clone();
        let mut destination_stack = stacks.get_mut(destination - 1).unwrap().clone();
        // Move crates one by one, from source stack to destination stack
        for _ in 0..volume {
            let c = source_stack.pop_front().unwrap();
            destination_stack.push_front(c);
        }
        // Replace original source and destination stacks with their modified versions
        stacks[source - 1] = source_stack.to_owned();
        stacks[destination - 1] = destination_stack.to_owned();
    }

    // println!("{:#?}", stacks);

    // Get top crates for Part 1
    let top_crates = get_top_crates(stacks);

    println!("Part 1: {}", top_crates);

    // ----------------------------------------------------------------------

    // Part 2: Retain order of crates moved within a single instruction
    // Apply instructions to cargo 'orginal_stacks'
    // Use orginal_stacks, as the unmodified original parse stacks
    let mut stacks2 = orginal_stacks;

    for (volume, source, destination) in parsed_instructions {
        let mut source_stack = stacks2.get_mut(source - 1).unwrap().clone();
        let mut destination_stack = stacks2.get_mut(destination - 1).unwrap().clone();
        // Staging deque used to pull the top 'volume' crates off the source stack
        // and store them until we push each crate back onto the destination stack
        let mut staging_deque: VecDeque<char> = VecDeque::new();
        // Pop crates off of source stack, and store in staging area
        for _ in 0..volume {
            let c = source_stack.pop_front().unwrap();
            staging_deque.push_front(c);
        }
        // Push staging area crates onto destination stack
        for _ in 0..volume {
            let c = staging_deque.pop_front().unwrap();
            destination_stack.push_front(c);
        }
        // Replace original source and destination stacks with their modified versions
        stacks2[source - 1] = source_stack.to_owned();
        stacks2[destination - 1] = destination_stack.to_owned();
    }

    println!("Part 2: {}", get_top_crates(stacks2));
}
