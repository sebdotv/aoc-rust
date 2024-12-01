use crate::challenge::Day;
use anyhow::Result;
use indexmap::IndexMap;
use itertools::Itertools;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (11, Some(3569916)),
        part2_solutions: Some((31, None)),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let (mut a, mut b) = parse_input(data);
    a.sort_unstable();
    b.sort_unstable();
    let sum = a.iter().zip(b.iter()).map(|(x, y)| x.abs_diff(*y)).sum();
    Ok(sum)
}

fn part2(data: &str) -> Result<usize> {
    let (a, b) = parse_input(data);

    let mut counts: IndexMap<usize, usize> = IndexMap::new();
    for b in b {
        *counts.entry(b).or_insert(0) += 1;
    }

    let sum = a
        .iter()
        .map(|x| x * counts.get(x).copied().unwrap_or(0))
        .sum();

    Ok(sum)
}

fn parse_input(data: &str) -> (Vec<usize>, Vec<usize>) {
    data.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}
