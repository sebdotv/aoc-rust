use anyhow::Result;
use itertools::Itertools;

use crate::challenge::Day;

pub fn day() -> Day<i32> {
    Day {
        part1_solutions: (114, Some(1731106378)),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<i32> {
    let sum = data
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
        })
        .map(|values| compute_next_value(values))
        .sum();
    Ok(sum)
}

fn compute_next_value(values: Vec<i32>) -> i32 {
    if values.iter().all(|&v| v == 0) {
        0
    } else {
        let next_layer = values
            .iter()
            .zip(values.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect_vec();
        let diff = compute_next_value(next_layer);
        values.last().unwrap() + diff
    }
}

fn part2(_data: &str) -> Result<i32> {
    todo!()
}
