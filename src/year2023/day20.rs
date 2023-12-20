use anyhow::{bail, Result};
use itertools::Itertools;
use std::str::FromStr;

use crate::challenge::Day;

pub fn day() -> Day<i32> {
    Day {
        part1_solutions: (32000000, None),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<i32> {
    let puzzle: Puzzle = data.parse()?;
    dbg!(puzzle);
    Ok(0)
}

fn part2(_data: &str) -> Result<i32> {
    todo!()
}

#[derive(Debug)]
struct Puzzle {
    modules: Vec<ModuleConfig>,
}
impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        s.lines()
            .map(str::parse)
            .collect::<Result<Vec<_>>>()
            .map(|modules| Self { modules })
    }
}
#[derive(Debug)]
struct ModuleConfig {
    r#type: ModuleType,
    name: String,
    destinations: Vec<String>,
}
#[derive(Debug)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}
impl FromStr for ModuleConfig {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (left, destinations) = s.split_once(" -> ").unwrap();
        let (r#type, name) = if left == "broadcaster" {
            (ModuleType::Broadcast, "broadcaster")
        } else if let Some(name) = left.strip_prefix('%') {
            (ModuleType::FlipFlop, name)
        } else if let Some(name) = left.strip_prefix('&') {
            (ModuleType::Conjunction, name)
        } else {
            bail!("invalid module type: {}", left);
        };
        let name = name.to_owned();
        let destinations = destinations
            .split(", ")
            .map(ToOwned::to_owned)
            .collect_vec();
        Ok(Self {
            r#type,
            name,
            destinations,
        })
    }
}
