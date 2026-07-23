pub fn main() {
    let input = include_str!("../input.txt");

    let sol: u32 = input
        .split("\n\n")
        .map(|group| {
            group
                .bytes()
                .filter(|c| c.is_ascii_alphabetic())
                .fold(0u32, |acc, c| acc | (1 << (c - b'a')))
                .count_ones()
        })
        .sum();

    println!("{sol}");
}
