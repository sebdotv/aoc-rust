use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;

use crate::challenge::ChallengeDay;

pub fn day() -> ChallengeDay<u32> {
    ChallengeDay {
        part1_solutions: (142, Some(54390)),
        part2_solutions: Some((281, None)),
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
    re: Regex,
}
impl Part2Extractor {
    fn new() -> Self {
        let digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let re = format!("[0-9]|{}", digits.join("|"));
        let re = Regex::new(re.as_str()).unwrap();
        Self { digits, re }
    }

    fn process(&self, s: &str) -> Result<(u32, u32)> {
        let matches = self.re.find_iter(s).map(|m| m.as_str()).collect_vec();
        let first: u32 = self.parse_digit(matches.first().unwrap())?;
        let last: u32 = self.parse_digit(matches.last().unwrap())?;
        println!(
            "line: {}; matches: {:?}; result: {:?}",
            s,
            matches,
            (first, last)
        );
        Ok((first, last))
    }

    fn parse_digit(&self, s: &str) -> Result<u32> {
        if let Ok(n) = s.parse::<u32>() {
            Ok(n)
        } else {
            let n = self
                .digits
                .iter()
                .position(|&d| d == s)
                .ok_or(anyhow!("not a spelled digit"))?
                + 1;
            Ok(n as u32)
        }
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
