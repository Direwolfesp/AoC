//! This implementation is very naive and actually does not take into
//! account that we can represent each code as a binary number >_< .
//! This will be fixed for part 2.

#[derive(Copy, Clone)]
pub enum Partition {
    Upper,
    Lower,
}

impl Partition {
    fn from_row(row: char) -> Option<Self> {
        match row {
            'F' => Some(Self::Lower),
            'B' => Some(Self::Upper),
            _ => None,
        }
    }

    fn from_column(col: char) -> Option<Self> {
        match col {
            'L' => Some(Self::Lower),
            'R' => Some(Self::Upper),
            _ => None,
        }
    }
}

fn search(len: u64, tokens: &[Partition]) -> u64 {
    let mut start = 0;
    let mut end = len - 1;
    let mut mid = start + end / 2;

    for &tok in tokens {
        match tok {
            Partition::Upper => start = mid,
            Partition::Lower => end = mid,
        };
        mid = (start + end + 1) / 2;
    }

    match tokens.last().expect("cant be empty") {
        Partition::Lower => mid - 1,
        Partition::Upper => mid,
    }
}

fn parse_tokens(line: &str) -> Vec<Partition> {
    line.chars()
        .enumerate()
        .take(10)
        .filter_map(|(i, c)| {
            if i < 7 {
                Partition::from_row(c)
            } else {
                Partition::from_column(c)
            }
        })
        .collect()
}

pub fn main() {
    let input = include_str!("../input.txt");

    let sol = input
        .lines()
        .map(|line| {
            let tokens = parse_tokens(line);
            let rows = search(128, &tokens[0..7]);
            let cols = search(8, &tokens[7..10]);
            rows * 8 + cols
        })
        .max()
        .expect("input is not empty");

    println!("Sol is {sol}");
}

#[cfg(test)]
mod tests {
    use crate::{parse_tokens, search};

    #[test]
    fn test_input() {
        let line = include_str!("../test.txt").lines().next().unwrap();
        let tokens = parse_tokens(line);
        let rows = search(128, &tokens[0..7]);
        let cols = search(8, &tokens[7..10]);
        let res = rows * 8 + cols;
        assert_eq!(res, 357)
    }

    #[test]
    fn search_row() {
        let path = parse_tokens("FBFBBFF");
        let row = search(128, &path);
        assert_eq!(row, 44);
    }
}
