use std::collections::{HashMap, HashSet};

pub fn main() {
    let input = include_str!("../input.txt");

    let sol: usize = input
        .split("\n\n")
        .map(|group| (group, group.lines().count()))
        .map(|(group, num_persons)| {
            group
                .bytes()
                .filter(|byte| byte.is_ascii_alphabetic())
                .fold(HashMap::new(), |mut acc, byte| {
                    let count = acc.entry(byte).or_insert(0);
                    *count += 1;
                    acc
                })
                .into_iter()
                .filter(|&(_, count)| count as usize == num_persons)
                .count()
        })
        .sum();

    println!("{sol}");
}

#[allow(dead_code)]
/// A more idiomatic implementation converting each line into a HasSet
/// and computing the intersection of all the lines together, and
/// counting the number of elements
fn alt_main() {
    let init: HashSet<u8> = (b'a'..=b'z').collect();

    let sol: usize = include_str!("../input.txt")
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.as_bytes().iter().copied().collect())
                .fold(init.clone(), |acc, x| {
                    acc.intersection(&x).copied().collect()
                })
                .len()
        })
        .sum();

    println!("{sol}");
}
