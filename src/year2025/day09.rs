use crate::challenge::Day;
use anyhow::Result;
use itertools::Itertools;
use std::cmp::{max, min};

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (50, Some(4750297200)),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let locs = parse(data);
    let max = locs
        .iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| {
            let min_x = min(x1, x2);
            let min_y = min(y1, y2);
            let max_x = max(x1, x2);
            let max_y = max(y1, y2);
            (max_x - min_x + 1) * (max_y - min_y + 1)
        })
        .max()
        .unwrap();
    Ok(max as usize)
}

fn part2(_data: &str) -> Result<usize> {
    todo!()
}

fn parse(data: &str) -> Vec<(usize, usize)> {
    data.lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}
