use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

use crate::challenge::Day;

pub fn day() -> Day<i32> {
    Day {
        part1_solutions: (114, Some(1731106378)),
        part2_solutions: Some((2, Some(1087))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<i32> {
    sum_of_next_values(data, &Side::Right)
}

fn part2(data: &str) -> Result<i32> {
    sum_of_next_values(data, &Side::Left)
}

fn sum_of_next_values(data: &str, side: &Side) -> Result<i32> {
    let sum = data
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().with_context(|| anyhow!("parse error")))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(|values| compute_next_value(values, side))
        .sum();
    Ok(sum)
}

enum Side {
    Right,
    Left,
}

fn compute_next_value(values: &[i32], side: &Side) -> i32 {
    if values.iter().all(|&v| v == 0) {
        0
    } else {
        let next_layer = values
            .iter()
            .zip(values.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect_vec();
        let diff = compute_next_value(&next_layer, side);
        match side {
            Side::Right => values.last().unwrap() + diff,
            Side::Left => values.first().unwrap() - diff,
        }
    }
}
