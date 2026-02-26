use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

fn main() {
    let (_, file) = env::args()
        .enumerate()
        .filter(|(i, _)| *i == 1)
        .next()
        .expect("Usage: ./main <file>");

    let res = part_a(&file).expect("Failed part a");
    println!("{res}");
}

fn part_a(file: &str) -> Result<i32, std::io::Error> {
    let file = File::open(file)?;

    let mut list_l: Vec<i32> = Vec::new();
    let mut list_r: Vec<i32> = Vec::new();

    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| {
            let mut nums = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
            list_l.push(nums.next().unwrap());
            list_r.push(nums.next().unwrap());
        });

    list_l.sort();
    list_r.sort();

    let total = iter::zip(list_l, list_r)
        .map(|(left, right)| (left - right).abs())
        .sum();

    Ok(total)
}
