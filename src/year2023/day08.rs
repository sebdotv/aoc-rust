use std::str::FromStr;

use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use itertools::Itertools;
use prime_factorization::Factorization;
use strum_macros::EnumString;

use crate::challenge::Day;

pub fn day() -> Day<u64> {
    Day {
        part1_solutions: (2, Some(19099)),
        part2_solutions: Some((6, Some(17099847107071))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: true,
    }
}

fn part1(data: &str) -> Result<u64> {
    let puzzle = data.parse::<Puzzle>()?;
    let start = puzzle.network.get("AAA").unwrap();
    solve(&puzzle, start, |node_id| node_id == "ZZZ")
}

fn solve(puzzle: &Puzzle, start: &Node, end_condition: fn(&str) -> bool) -> Result<u64> {
    let mut node = start;
    let mut instr_idx = 0;
    let mut steps = 1;

    loop {
        use Instruction::*;

        let instr = puzzle.instructions.get(instr_idx).unwrap();

        let node_id = match instr {
            L => &node.left,
            R => &node.right,
        };
        if end_condition(node_id) {
            break;
        }

        node = puzzle
            .network
            .get(node_id)
            .ok_or(anyhow!("missing node {}", node_id))?;

        instr_idx = (instr_idx + 1) % puzzle.instructions.len();

        steps += 1;
    }

    Ok(steps)
}

fn part2(data: &str) -> Result<u64> {
    let puzzle = data.parse::<Puzzle>()?;
    let starts = puzzle
        .network
        .values()
        .filter(|node| node.id.ends_with('A'))
        .collect_vec();
    let steps = starts
        .iter()
        .map(|start| solve(&puzzle, start, |node_id| node_id.ends_with('Z')).unwrap())
        .collect_vec();
    let unique_factors_product = steps
        .iter()
        .flat_map(|&x| Factorization::run(x).factors)
        .unique()
        .product();
    Ok(unique_factors_product)
}

#[derive(Debug)]
struct Puzzle {
    instructions: Vec<Instruction>,
    network: IndexMap<String, Node>,
}
#[derive(Debug, EnumString)]
enum Instruction {
    L,
    R,
}
#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl FromStr for Puzzle {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        let (instructions, network) = lines
            .split(|line| line.is_empty())
            .collect_tuple()
            .ok_or(anyhow!("Could not split"))?;
        let (instructions,) = instructions.iter().collect_tuple().unwrap();
        let instructions = instructions
            .chars()
            .map(|c| c.to_string())
            .map(|s| s.parse::<Instruction>().unwrap())
            .collect();
        let network = network
            .iter()
            .map(|s| s.parse::<Node>().unwrap())
            .map(|node| (node.id.clone(), node))
            .collect();
        Ok(Self {
            instructions,
            network,
        })
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (id, rhs) = s.split_once(" = ").unwrap();
        let (left, right) = rhs
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        Ok(Self {
            id: id.to_owned(),
            left: left.to_owned(),
            right: right.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_extra_examples() -> Result<()> {
        let f = |s: &str| part1(s.trim().lines().map(str::trim).join("\n").as_str());
        assert_eq!(
            f(r"
                LLR
                
                AAA = (BBB, BBB)
                BBB = (AAA, ZZZ)
                ZZZ = (ZZZ, ZZZ)
            ")?,
            6
        );
        Ok(())
    }
}
