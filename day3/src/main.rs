use std::{
    collections::{HashMap, HashSet},
    iter::zip,
};

fn find_common_items(left: Vec<char>, right: Vec<char>) -> Vec<char> {
    let pocket1: HashSet<char> = left.into_iter().map(|x| x).collect();
    let pocket2: HashSet<char> = right.into_iter().map(|x| x).collect();

    (*pocket1
        .intersection(&pocket2)
        .into_iter()
        .map(|x| *x)
        .collect::<Vec<char>>())
    .to_vec()
}

fn main() {
    // Part 1

    // Construct priority value lookup hashmap
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let prior: Vec<i32> = (1..53).collect();
    let lookup_pairs = zip(alphabet, prior);

    let priority_lookup: HashMap<char, i32> = HashMap::from_iter(lookup_pairs);

    // Main logic
    let priority: i32 = include_str!("day3.txt")
        .lines()
        .map(|group| {
            let group_vec: Vec<char> = group.chars().collect();
            let (head, tail) = group_vec.split_at(group_vec.len() / 2);
            let l = head.to_owned();
            let r = tail.to_owned();
            (l, r)
        })
        .map(|pair: (Vec<char>, Vec<char>)| {
            find_common_items(pair.0, pair.1).first().unwrap().clone()
        })
        .map(|ch| priority_lookup.get(&ch).unwrap())
        .sum();

    println!("Part 1: {:#?}", priority);

    // Part 2
    let group_priority: i32 = include_str!("day3.txt")
        .lines()
        .map(|group| {
            let group_vec: Vec<char> = group.chars().collect();
            group_vec
        })
        .collect::<Vec<Vec<char>>>()
        .chunks(3)
        .into_iter()
        .map(|triple| {
            triple.into_iter().fold(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                    .chars()
                    .collect(),
                |acc, x| find_common_items(acc, x.to_owned()),
            )
        })
        .into_iter()
        .map(|ch| priority_lookup.get(&ch.first().unwrap()).unwrap())
        .sum();

    println!("Part 2: {:#?}", group_priority);
}
