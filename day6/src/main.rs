use std::collections::{HashSet, VecDeque};
use std::str::Chars;


fn find_start_of_packet_marker(datastream: Chars, window_size: usize) -> usize {
    let mut window: VecDeque<char> = VecDeque::new();

    for (i, c) in datastream.enumerate() {
        window.push_front(c);
        if window.len() > window_size {
            window.pop_back();
        }
        let set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
        if set.len() == window_size {
            return i+1;
        }
    }
    return 0
}


fn main() {
    
    let datastream = include_str!("day6.txt").chars();
    
    // Part 1
    let part1 = find_start_of_packet_marker(datastream.clone(), 4);
    println!("Part 1: {}", part1);

    // Part 2
    let part2 = find_start_of_packet_marker(datastream.clone(), 14);
    println!("Part 2: {}", part2);

}
