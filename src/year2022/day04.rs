use std::ops::Range;

use anyhow::{anyhow, Context, Result};

use crate::challenge::ChallengeDay;

pub fn day() -> ChallengeDay<u32> {
    ChallengeDay {
        part1_solutions: (2, Some(569)),
        part2_solutions: Some((4, Some(936))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<u32> {
    let assignments = parse_assignments(data)?;
    let full_overlaps = assignments
        .iter()
        .filter(|(e1, e2)| fully_contains(e1, e2) || fully_contains(e2, e1))
        .count();
    Ok(full_overlaps as u32)
}

fn parse_assignments(data: &str) -> Result<Vec<(Range<u32>, Range<u32>)>> {
    let assignments = data
        .lines()
        .map(|line| {
            let (elf1, elf2) = line.split_once(',').ok_or(anyhow!("invalid line"))?;
            let (elf1, elf2) = (parse_range(elf1)?, parse_range(elf2)?);
            Ok((elf1, elf2))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(assignments)
}

fn parse_range(s: &str) -> Result<Range<u32>> {
    let parse_u32 = |s: &str| {
        s.parse::<u32>()
            .with_context(|| format!("Could not parse value {}", s))
    };
    let (from, to) = s.split_once('-').ok_or(anyhow!("Could not split range"))?;
    Ok(parse_u32(from)?..parse_u32(to)?)
}

fn fully_contains(r: &Range<u32>, other: &Range<u32>) -> bool {
    r.start <= other.start && r.end >= other.end
}
fn overlaps(r: &Range<u32>, other: &Range<u32>) -> bool {
    r.start <= other.end && r.end >= other.start
}

fn part2(data: &str) -> Result<u32> {
    let assignments = parse_assignments(data)?;
    let full_overlaps = assignments
        .iter()
        .filter(|(e1, e2)| overlaps(e1, e2) || overlaps(e2, e1))
        .count();
    Ok(full_overlaps as u32)
}
