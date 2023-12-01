use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::challenge::ChallengeDay;

pub fn day() -> ChallengeDay<u32> {
    ChallengeDay {
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

struct Part2Extractor {
    digits: [&'static str; 9],
}
impl Part2Extractor {
    fn new() -> Self {
        let digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        Self { digits }
    }

    fn get_digit(&self, chars: &Vec<char>, pos: usize) -> Option<u32> {
        let c = chars[pos];
        if c.is_ascii_digit() {
            Some(c.to_digit(10).unwrap())
        } else {
            for (i, digit) in self.digits.iter().enumerate() {
                let substr: String = chars.iter().skip(pos).take(digit.len()).collect();
                if substr == *digit {
                    return Some(i as u32 + 1);
                }
            }
            None
        }
    }

    fn process(&self, s: &str) -> Result<(u32, u32)> {
        let chars = s.chars().collect_vec();
        let digits = (0..chars.len())
            .filter_map(|pos| self.get_digit(&chars, pos))
            .collect_vec();
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        Ok((*first, *last))
    }
}

fn part2(data: &str) -> Result<u32> {
    let extractor = Part2Extractor::new();
    let values = data
        .lines()
        .map(|line| {
            let (first, last) = extractor.process(line)?;
            Ok(first * 10 + last)
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(values.iter().sum())
}

fn first_digit<I>(mut chars: I) -> Option<u32>
where
    I: Iterator<Item = char>,
{
    let char = chars.find(|c| c.is_ascii_digit())?;
    let digit = char.to_digit(10).expect("unexpected");
    Some(digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_extractor_works() {
        let data = "6twodndmhcgxlgbqbqndbbthnngblfgtzh5fouroneightrjp";
        let (first, last) = Part2Extractor::new().process(data).unwrap();
        assert_eq!((first, last), (6, 8));
    }
}
