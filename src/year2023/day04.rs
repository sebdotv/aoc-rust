use std::iter::repeat;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use indexmap::IndexSet;
use lazy_static::lazy_static;
use regex::Regex;

use crate::challenge::ChallengeDay;

pub fn day() -> ChallengeDay<u32> {
    ChallengeDay {
        part1_solutions: (13, Some(27845)),
        part2_solutions: Some((30, None)),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<u32> {
    let cards = data
        .lines()
        .map(str::parse::<Card>)
        .collect::<Result<Vec<_>>>()?;
    let sum: u32 = cards
        .iter()
        .map(|card| {
            let count = card.count_winning();
            let points = if count == 0 { 0 } else { 1 << (count - 1) };
            points
        })
        .sum();
    Ok(sum)
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^Card\s+(\d+):\s+(.*)\s+\|\s+(.*)$").unwrap();
}

#[derive(Debug)]
struct Card {
    _id: u32,
    winning: IndexSet<u32>,
    numbers: IndexSet<u32>,
}
impl Card {
    fn count_winning(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(*n))
            .count()
    }
}
impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let caps = RE.captures(s).ok_or(anyhow!("invalid line: `{}`", s))?;
        let (_, [id, winning, numbers]) = caps.extract();
        let parse_u32 = |s: &str| {
            s.parse::<u32>()
                .map_err(|_| anyhow!("number parse error for `{}`", s))
        };
        let id = parse_u32(id)?;
        let split_numbers = |s: &str| {
            s.split_whitespace()
                .map(parse_u32)
                .collect::<Result<IndexSet<_>>>()
        };
        let winning = split_numbers(winning)?;
        let numbers = split_numbers(numbers)?;
        Ok(Card {
            _id: id,
            winning,
            numbers,
        })
    }
}

fn part2(data: &str) -> Result<u32> {
    let cards = data
        .lines()
        .map(str::parse::<Card>)
        .collect::<Result<Vec<_>>>()?;
    let mut copies = repeat(0u32).take(cards.len()).collect::<Vec<_>>();
    for (i, card) in cards.iter().enumerate() {
        let winning = card.count_winning();
        let instances = 1 + copies[i];
        // println!(
        //     "processing card {}: winning={}, instances={}",
        //     card._id, winning, instances
        // );
        for (j, v) in copies.iter_mut().skip(i + 1).take(winning).enumerate() {
            *v += instances;
            // println!("  added {} copies to card {}", instances, cards[j]._id);
        }
    }
    let count = cards.len() as u32 + copies.iter().sum::<u32>();
    Ok(count)
}
