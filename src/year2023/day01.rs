use anyhow::{anyhow, Result};

use crate::challenge::ChallengeDay;

pub fn day() -> ChallengeDay<u32> {
    ChallengeDay {
        part1_solutions: (142, Some(54390)),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
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

fn part2(_data: &str) -> Result<u32> {
    todo!()
}

fn first_digit<I>(mut chars: I) -> Option<u32>
where
    I: Iterator<Item = char>,
{
    let char = chars.find(|c| c.is_ascii_digit())?;
    let digit = char.to_digit(10).expect("unexpected");
    Some(digit)
}
