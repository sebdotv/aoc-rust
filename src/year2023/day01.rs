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

fn part2(data: &str) -> Result<u32> {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let re = format!("[0-9]|{}", digits.join("|"));
    let re = Regex::new(re.as_str()).unwrap();
    let parse = |s: &str| -> Result<u32> {
        if let Ok(n) = s.parse::<u32>() {
            Ok(n)
        } else {
            let n = digits
                .iter()
                .position(|&d| d == s)
                .ok_or(anyhow!("not a spelled digit"))?
                + 1;
            Ok(n as u32)
        }
    };
    let values = data
        .lines()
        .map(|line| {
            let matches = re.find_iter(line).map(|m| m.as_str()).collect_vec();
            // println!("line: {}; matches: {:?}", line, matches);
            let first: u32 = parse(matches.first().unwrap()).unwrap();
            let last: u32 = parse(matches.last().unwrap()).unwrap();
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
