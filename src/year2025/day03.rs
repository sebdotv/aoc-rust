use crate::challenge::Day;
use anyhow::Result;
use itertools::Itertools;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (357, Some(17092)),
        part2_solutions: Some((3121910778619, Some(170147128753455))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let sum = data.lines().map(|line| best(line, 2)).sum();
    Ok(sum)
}

fn part2(data: &str) -> Result<usize> {
    let sum = data.lines().map(|line| best(line, 12)).sum();
    Ok(sum)
}

fn best(line: &str, n: usize) -> usize {
    let digits = line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
    let mut best = Vec::new();
    let mut idx = 0;
    for i in 0..n {
        // find the highest digit, with preference to earlier digits
        // leave enough digits left to fill the rest
        let mut best_digit = 0;
        let mut best_idx = None;
        for (j, &digit) in digits
            .iter()
            .enumerate()
            .take(digits.len() - n + i + 1)
            .skip(idx)
        {
            if digit > best_digit {
                best_digit = digit;
                best_idx = Some(j);
            }
        }
        best.push(best_digit);
        idx = best_idx.unwrap() + 1;
    }
    best.iter().fold(0, |acc, &d| acc * 10 + d as usize)
}
