use anyhow::Result;
use anyhow::{anyhow, Context};
use itertools::Itertools;

use crate::challenge::ChallengeDay;

pub fn day() -> ChallengeDay<u32> {
    ChallengeDay {
        part1_solutions: (24000, Some(70613)),
        part2_solutions: Some((45000, Some(205805))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<u32> {
    let groups = get_groups(data)?;
    let max = groups.iter().max().ok_or(anyhow!("Could not find max"))?;
    Ok(*max)
}

fn part2(data: &str) -> Result<u32> {
    let groups = get_groups(data)?;
    let sum = groups.iter().sorted().rev().take(3).sum::<u32>();
    Ok(sum)
}

fn get_groups(data: &str) -> Result<Vec<u32>> {
    let lines = data.lines().collect_vec();
    let groups = lines.split(|line| line.is_empty()).collect_vec();
    let groups = groups
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|line| {
                    line.parse::<u32>()
                        .with_context(|| format!("Could not parse {}", line))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;
    let groups = groups
        .iter()
        .map(|group| group.iter().sum::<u32>())
        .collect_vec();
    Ok(groups)
}
