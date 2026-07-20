const TARGET_SUM: i32 = 2020;

use itertools::{self, Itertools};

// 1721
// 979
// 366
// 299
// 675
// 1456
fn main() {
    let [a, b, c] = include_str!("../input.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i32>>()
        .into_iter()
        .array_combinations()
        .find(|[a, b, c]| a + b + c == TARGET_SUM)
        .expect("No solution found");

    let sol = a * b * c;
    println!("Solution: {sol}");
}
