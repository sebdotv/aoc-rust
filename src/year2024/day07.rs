use crate::challenge::Day;
use anyhow::Result;
use pathfinding::prelude::bfs;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (3749, Some(538191549061)),
        part2_solutions: Some((11387, Some(34612812972206))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    run_part(data, false)
}

fn part2(data: &str) -> Result<usize> {
    run_part(data, true)
}

type Equation = (usize, Vec<usize>);

fn run_part(data: &str, with_concat: bool) -> Result<usize> {
    Ok(data
        .lines()
        .map(parse_equation)
        .filter_map(|eq| solve_equation(&eq, with_concat).then_some(eq.0))
        .sum())
}

fn parse_equation(line: &str) -> Equation {
    let (left, right) = line.split_once(": ").unwrap();
    let left = left.parse().unwrap();
    let right = right.split(' ').map(|s| s.parse().unwrap()).collect();
    (left, right)
}

fn solve_equation(eq: &Equation, with_concat: bool) -> bool {
    #[derive(EnumIter, Debug, Eq, PartialEq, Hash, Copy, Clone)]
    enum Operator {
        Add,
        Multiply,
        Concat,
    }

    #[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
    struct Pos {
        offset: usize,
        value: usize,
        op: Option<Operator>,
    }

    let (left, right) = eq;

    let start = Pos {
        offset: 0,
        value: right[0],
        op: None,
    };
    let successors = |pos: &Pos| -> Vec<Pos> {
        if pos.offset == right.len() - 1 {
            return vec![];
        }

        // micro optimization: cut early if the value is already too big
        if pos.value > *left {
            return vec![];
        }

        Operator::iter()
            .filter(|op| with_concat || *op != Operator::Concat)
            .map(|op| {
                let next_offset = pos.offset + 1;
                let a = pos.value;
                let b = right[next_offset];
                let value = match op {
                    Operator::Add => a + b,
                    Operator::Multiply => a * b,
                    Operator::Concat => format!("{}{}", a, b).parse().unwrap(),
                };
                Pos {
                    offset: next_offset,
                    op: Some(op),
                    value,
                }
            })
            .collect()
    };
    let success = |pos: &Pos| pos.offset == right.len() - 1 && pos.value == *left;

    bfs(&start, successors, success).is_some()
}
