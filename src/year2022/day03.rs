use anyhow::{anyhow, Result};
use indexmap::IndexSet;
use itertools::Itertools;

use crate::challenge::ChallengeDay;

pub fn day() -> ChallengeDay<u32> {
    ChallengeDay {
        part1_solutions: (157, Some(7875)),
        part2_solutions: Some((70, Some(2479))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<u32> {
    let dupes = data
        .lines()
        .map(|line| {
            let compartments: [&str; 2] = line.split_at(line.len() / 2).into();
            let [c1, c2] = compartments.map(|c| c.chars().collect::<IndexSet<_>>());
            let intersection = c1.intersection(&c2);
            if let Some((item,)) = intersection.collect_tuple::<(&char,)>() {
                Ok(*item)
            } else {
                Err(anyhow!("unexpected intersection"))
            }
        })
        .collect::<Result<Vec<_>>>()?;
    total_priority(dupes)
}

fn part2(data: &str) -> Result<u32> {
    let commons = data
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let x = chunk
                .map(|line| line.chars().collect::<IndexSet<_>>())
                .reduce(|acc, e| acc.intersection(&e).copied().collect::<IndexSet<_>>())
                .ok_or_else(|| anyhow!("unexpected sets"))?;
            let (common_char,) = x
                .iter()
                .collect_tuple::<(&char,)>()
                .ok_or_else(|| anyhow!("unexpected intersection"))?;
            Ok(*common_char)
        })
        .collect::<Result<Vec<char>>>()?;
    total_priority(commons)
}

fn total_priority(chars: Vec<char>) -> Result<u32> {
    let priorities = chars
        .iter()
        .map(|c| match c {
            'a'..='z' => Ok(*c as u32 - 'a' as u32 + 1),
            'A'..='Z' => Ok(*c as u32 - 'A' as u32 + 27),
            _ => Err(anyhow!("unexpected char")),
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(priorities.iter().sum())
}
