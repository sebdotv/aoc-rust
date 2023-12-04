use anyhow::{anyhow, Result};
use indexmap::IndexSet;
use lazy_static::lazy_static;
use regex::Regex;

use crate::challenge::ChallengeDay;

pub fn day() -> ChallengeDay<u32> {
    ChallengeDay {
        part1_solutions: (13, None),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<u32> {
    let cards = data.lines().map(parseLine).collect::<Result<Vec<_>>>()?;
    let sum: u32 = cards
        .iter()
        .map(|card| {
            let count = card
                .numbers
                .iter()
                .filter(|n| card.winning.contains(*n))
                .count();
            let points = if count == 0 { 0 } else { 1 << (count - 1) };
            points
        })
        .sum();
    Ok(sum as u32)
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^Card\s+(\d+):\s+(.*)\s+\|\s+(.*)$").unwrap();
}

fn parseLine(s: &str) -> Result<Card> {
    let caps = RE.captures(s).ok_or(anyhow!("invalid line: `{}`", s))?;
    let (_, [id, winning, numbers]) = caps.extract();
    let parse_u32 = |s: &str| {
        s.parse::<u32>()
            .map_err(|_| anyhow!("number parse error for `{}`", s))
    };
    let id = parse_u32(id)?;
    let split_numbers = |s: &str| {
        s.split_whitespace()
            // .map(|s| s.trim())
            // .filter(|s| !s.is_empty())
            .map(parse_u32)
            .collect::<Result<IndexSet<_>>>()
    };
    let winning = split_numbers(winning)?;
    let numbers = split_numbers(numbers)?;
    Ok(Card {
        id,
        winning,
        numbers,
    })
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning: IndexSet<u32>,
    numbers: IndexSet<u32>,
}

fn part2(_data: &str) -> Result<u32> {
    todo!()
}
