use std::collections::VecDeque;

/// Day 10: ...

/// Update the pixel related to the 'current' cycle on the CRT
fn update_pixel(x: i32, cycle: i32, pixels: &mut Vec<char>) {
    if x-1 == (cycle-1) % 40 || x == (cycle-1) % 40 || x+1 == (cycle-1) % 40 {
        pixels.push('#');
    } else {
        pixels.push('.');
    }
}

/// Update signal strength
fn update_signal_strength(cycle: i32, signal_strength: &mut i32, x: i32) {
    if cycle % 40 - 20 == 0 {
        *signal_strength += cycle * x;
    }
}

fn main() {
    
    // Read in operations, and queue them in a VecDeque so we can pop them off one by one.
    let operations: Vec<&str> = include_str!("day10.txt").lines().collect();
    let mut queued_operations = VecDeque::from(operations);

    // Initalise clock cycle, counter x, and signal_strength.
    let mut cycle = 0;
    let mut x = 1;
    let mut signal_strength = 0;
    let mut pixels = Vec::<char>::new();

    // Begin operation loop, within which we will apply each operation in turn.
    while queued_operations.len() > 0 {

        // Pull operation of queue
        let op = queued_operations
            .pop_front()
            .unwrap();
        

        // Determine changes required to cycle and x
        _ = match op {
            "noop" => { 
                cycle += 1;
                update_signal_strength(cycle, &mut signal_strength, x);
                update_pixel(x, cycle, &mut pixels);
             },
            addx_op => {
                let (_, inc) = addx_op.split_once(" ").unwrap();
                let inc_x = inc.parse::<i32>().unwrap();

                cycle += 1;
                update_signal_strength(cycle, &mut signal_strength, x);
                update_pixel(x, cycle, &mut pixels);

                cycle += 1;
                update_signal_strength(cycle, &mut signal_strength, x);
                update_pixel(x, cycle, &mut pixels);
                x += inc_x;

            }
        };

    }

    println!("Part 1: {}", signal_strength);

    // Part 2: CRT screen
    let line1: String = pixels[0..39].into_iter().collect();
    let line2: String = pixels[40..79].into_iter().collect();
    let line3: String = pixels[80..119].into_iter().collect();
    let line4: String = pixels[120..159].into_iter().collect();
    let line5: String = pixels[160..199].into_iter().collect();
    let line6: String = pixels[200..239].into_iter().collect();

    println!("{:?}", line1);
    println!("{:?}", line2);
    println!("{:?}", line3);
    println!("{:?}", line4);
    println!("{:?}", line5);
    println!("{:?}", line6);





}
