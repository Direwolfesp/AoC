//! Here we realized that the problem is literally about
//! parsing some binary number from a string, with different chars.
//! The actual trick is that, the whole number is 7 + 3 bits, and to get
//! the ID, we multiple * 8 the 7 bit part, which is effectively just
//! shifting a 7 bit number 3 positions to make space for the 3 bits.
//! So we can just create a 16 bit number right away.

use bitvec::prelude::*;

#[derive(Clone, Copy, Default, Debug, PartialEq, Ord, Eq, PartialOrd)]
struct Seat(u16);

impl Seat {
    fn parse(input: &str) -> Self {
        let mut res = Self::default();
        let bits = BitSlice::<u16>::from_element_mut(&mut res.0);

        for (i, &b) in input.as_bytes().iter().rev().enumerate() {
            bits.set(
                i,
                match b {
                    b'F' | b'L' => false,
                    b'B' | b'R' => true,
                    _ => panic!("bad character {}", b),
                },
            );
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Seat;

    #[test]
    fn test_seat_id() {
        assert_eq!(Seat::parse("FBFBBFFRLR"), Seat(357));
        assert_eq!(Seat::parse("FFFBBBFRRR"), Seat(119));
        assert_eq!(Seat::parse("BBFFBBFRLL"), Seat(820));
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../input.txt");

        let max = input
            .lines()
            .map(Seat::parse)
            .map(|s| s.0)
            .max()
            .expect("input is not empty");

        assert_eq!(max, 955);
    }
}

fn main() {
    let mut seats: Vec<Seat> = include_str!("../input.txt")
        .lines()
        .map(Seat::parse)
        .collect();

    seats.sort();

    let mut last_seat: Option<Seat> = None;
    for seat in seats {
        if let Some(last_seat) = last_seat {
            let gap = seat.0 - last_seat.0;
            if gap > 1 {
                println!("found seat: {}", last_seat.0 + 1);
                return;
            }
        }
        last_seat = Some(seat);
    }
}
