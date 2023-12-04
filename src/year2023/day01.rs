use anyhow::{anyhow, Result};
use itertools::Itertools;
use lazy_static::lazy_static;

use crate::challenge::Day;

pub fn day() -> Day<u32> {
    Day {
        part1_solutions: (142, Some(54390)),
        part2_solutions: Some((281, Some(54277))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: true,
    }
}

fn part1(data: &str) -> Result<u32> {
    let values = data
        .lines()
        .map(|line| {
            let chars = line.chars();
            let chars_rev = line.chars().rev();
            let (first, last) = (
                first_digit(chars).ok_or(anyhow!("no first digit"))?,
                first_digit(chars_rev).ok_or(anyhow!("no last digit"))?,
            );
            Ok(first * 10 + last)
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(values.iter().sum())
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

lazy_static! {
    static ref DIGITS_CHARS: [Vec<char>; 9] = DIGITS.map(|s| s.chars().collect_vec());
}

struct Part2Extractor {}
impl Part2Extractor {
    fn get_digit(chars: &[char], pos: usize) -> Option<u32> {
        let c = chars[pos];
        if c.is_ascii_digit() {
            Some(c.to_digit(10).unwrap())
        } else {
            for (i, digit) in DIGITS_CHARS.iter().enumerate() {
                if Self::slice_eq(chars, pos, digit) {
                    return Some(u32::try_from(i).unwrap() + 1);
                }
            }
            None
        }
    }

    fn slice_eq<T>(slice: &[T], pos: usize, other: &[T]) -> bool
    where
        T: Eq,
    {
        let len = other.len();
        if slice.len() < pos + len {
            return false;
        }
        for i in 0..len {
            if slice[pos + i] != other[i] {
                return false;
            }
        }
        true
    }

    fn process(s: &str) -> Result<(u32, u32)> {
        let chars = s.chars().collect_vec();
        let first = (0..chars.len())
            .find_map(|pos| Self::get_digit(&chars, pos))
            .ok_or(anyhow!("no first digit"))?;
        let last = (0..chars.len())
            .rev()
            .find_map(|pos| Self::get_digit(&chars, pos))
            .ok_or(anyhow!("no last digit"))?;
        Ok((first, last))
    }
}

fn part2(data: &str) -> Result<u32> {
    let values = data
        .lines()
        .map(|line| {
            let (first, last) = Part2Extractor::process(line)?;
            Ok(first * 10 + last)
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(values.iter().sum())
}

fn first_digit<I>(mut chars: I) -> Option<u32>
where
    I: Iterator<Item = char>,
{
    let char = chars.find(char::is_ascii_digit)?;
    let digit = char.to_digit(10).expect("unexpected");
    Some(digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_extractor_works() {
        let data = "6twodndmhcgxlgbqbqndbbthnngblfgtzh5fouroneightrjp";
        let (first, last) = Part2Extractor::process(data).unwrap();
        assert_eq!((first, last), (6, 8));
    }
}
