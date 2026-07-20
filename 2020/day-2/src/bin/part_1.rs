use std::ops::RangeInclusive;

use anyhow::bail;

struct PasswordPolicy {
    byte: u8,
    range: RangeInclusive<usize>,
}

impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        self.range.contains(
            &password
                .as_bytes()
                .iter()
                .copied()
                .filter(|&char| char == self.byte)
                .count(),
        )
    }
}

// 2-9 c: ccccccccc
fn parse_line(line: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    let mut words = line.split_whitespace().take(3);

    let mut range = words.next().unwrap().split('-');
    let min = range.next().ok_or(anyhow::anyhow!("policy range (min)"))?;
    let max = range.next().ok_or(anyhow::anyhow!("policy range (max)"))?;
    let range: RangeInclusive<usize> = RangeInclusive::new(min.parse()?, max.parse()?);

    let byte: u8 = {
        let letter_bytes = words.next().unwrap().as_bytes();
        if !letter_bytes.is_empty() {
            letter_bytes[0]
        } else {
            bail!("Invalid byte length")
        }
    };

    let Some(password) = words.next() else {
        bail!("Missing password")
    };

    Ok((PasswordPolicy { byte, range }, password))
}

// 1-3 a: abcde
// 1-3 b: cdefg
// How many passwords are valid?
pub fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");

    let valids = input
        .lines()
        .map(parse_line)
        .map(anyhow::Result::unwrap)
        .filter(|(pp, password)| pp.is_valid(password))
        .count();

    println!("Solution: {valids}");
    Ok(())
}
