use anyhow::bail;

#[derive(PartialEq, Debug)]
pub struct PasswordPolicy {
    byte: u8,
    range: [usize; 2],
}

impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        self.range
            .iter()
            .copied()
            .filter(|&index| password.as_bytes()[index] == self.byte)
            .count()
            == 1
    }
}

// 2-9 c: ccccccccc
pub fn parse_line(line: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    let mut words = line.split_whitespace().take(3);
    let mut range = words.next().unwrap().split('-');
    let min = range.next().ok_or(anyhow::anyhow!("policy range (min)"))?;
    let max = range.next().ok_or(anyhow::anyhow!("policy range (max)"))?;

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

    let min: usize = min.parse()?;
    let max: usize = max.parse()?;

    Ok((
        PasswordPolicy {
            byte,
            range: [min - 1, max - 1],
        },
        password,
    ))
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

#[cfg(test)]
mod tests {
    use crate::{PasswordPolicy, parse_line};

    #[test]
    fn test_is_valid() {
        let pp = PasswordPolicy {
            byte: b'a',
            range: [0, 2],
        };
        assert_eq!(pp.is_valid("abcde"), true, "'a' in position 1");
        assert_eq!(pp.is_valid("cdefg"), false, "no 'a' in either pos");
        assert_eq!(pp.is_valid("abade"), false, "'a' in positions 1 and 3");
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_line("1-3 a: abcde").unwrap(),
            (
                PasswordPolicy {
                    byte: b'a',
                    range: [0, 2],
                },
                "abcde",
            ),
        );
    }
}
