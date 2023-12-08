use crate::challenge::Day;
use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use itertools::Itertools;
use std::str::FromStr;
use strum_macros::EnumString;

pub fn day() -> Day<u32> {
    Day {
        part1_solutions: (2, None),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

const START: &str = "AAA";
const END: &str = "ZZZ";

fn part1(data: &str) -> Result<u32> {
    let puzzle = data.parse::<Puzzle>()?;
    println!("{:?}", puzzle);

    let mut node = puzzle.network.get(START).unwrap();
    let mut instr_idx = 0;
    let mut steps = 1;

    loop {
        use Instr::*;

        let instr = puzzle.instrs.get(instr_idx).unwrap();
        println!("{:?} {:?}", node, instr);

        let node_id = match instr {
            L => &node.left,
            R => &node.right,
        };

        if node_id == END {
            break;
        }

        node = puzzle.network.get(node_id).unwrap();

        instr_idx = (instr_idx + 1) % puzzle.instrs.len();

        // if instr_idx == 0 {
        steps += 1;
        // }
    }

    Ok(steps)
}

fn part2(_data: &str) -> Result<u32> {
    todo!()
}

#[derive(Debug)]
struct Puzzle {
    instrs: Vec<Instr>,
    network: IndexMap<String, Node>,
}
// #[derive(Debug, Eq, PartialEq, Hash, EnumString, strum_macros::Display, Copy, Clone)]
#[derive(Debug, EnumString)]
enum Instr {
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
        let (instrs, network) = lines
            .split(|line| line.is_empty())
            .collect_tuple()
            .ok_or(anyhow!("Could not split"))?;
        let (instrs,) = instrs.iter().collect_tuple().unwrap();
        let instrs = instrs
            .chars()
            .map(|c| c.to_string())
            .map(|s| s.parse::<Instr>().unwrap())
            .collect();
        let network = network
            .iter()
            .map(|s| s.parse::<Node>().unwrap())
            .map(|node| (node.id.clone(), node))
            .collect();
        Ok(Self { instrs, network })
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (id, rhs) = s.split_once(" = ").unwrap();
        let (left, right) = rhs
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
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
        let f = |s: &str| part1(s.trim().lines().map(|s| s.trim()).join("\n").as_str());
        assert_eq!(
            f(r#"
            LLR
            
            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "#)?,
            6
        );
        Ok(())
    }
}
