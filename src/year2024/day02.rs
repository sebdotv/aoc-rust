use crate::challenge::Day;
use anyhow::{Context, Result};
use itertools::Itertools;
use Safety::*;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (2, Some(572)),
        part2_solutions: Some((4, Some(612))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    Ok(data
        .lines()
        .filter(|line| {
            let levels = parse_levels(line).unwrap();
            eval_safety(&levels, false) == Safe
        })
        .count())
}

fn part2(data: &str) -> Result<usize> {
    Ok(data
        .lines()
        .filter(|line| {
            let levels = parse_levels(line).unwrap();
            eval_safety(&levels, true) != Unsafe
        })
        .count())
}

fn parse_levels(line: &&str) -> Result<Vec<usize>> {
    line.split_whitespace()
        .map(|s| s.parse::<usize>().context("invalid number"))
        .collect()
}

fn compute_diffs(levels: &[usize]) -> Vec<isize> {
    levels
        .iter()
        .zip(levels.iter().skip(1))
        .map(|(a, b)| isize::try_from(*b).unwrap() - isize::try_from(*a).unwrap())
        .collect()
}

/// # Returns
/// A vector of values for each difference in the input vector:
/// - `Some(true)` if the difference is positive
/// - `Some(false)` if the difference is negative
/// - `None` if the difference is not in the allowed range
fn compute_signs_from_diffs(diffs: &[isize]) -> Vec<Option<bool>> {
    let range = 1..=3;
    diffs
        .iter()
        .map(|diff| {
            if range.contains(&diff.abs()) {
                Some(*diff > 0)
            } else {
                None
            }
        })
        .collect()
}

fn eval_safety(levels: &[usize], with_problem_dampener: bool) -> Safety {
    use Safety::*;

    let diffs = compute_diffs(levels);
    let signs = compute_signs_from_diffs(&diffs);
    let counts = signs.iter().counts();

    let sorted_counts_desc = counts
        .iter()
        .sorted_by_key(|(_, count)| *count)
        .rev()
        .collect_vec();

    match sorted_counts_desc.first().unwrap() {
        (Some(_), count) if **count == signs.len() => Safe,
        (Some(_), _) if with_problem_dampener => {
            let (other_sign, _) = sorted_counts_desc.get(1).unwrap();
            let index = signs.iter().position(|s| s == **other_sign).unwrap();

            [index, index + 1]
                .iter()
                .filter_map(|index| {
                    let filtered_levels = levels
                        .iter()
                        .enumerate()
                        .filter_map(|(i, value)| if i == *index { None } else { Some(*value) })
                        .collect_vec();
                    if eval_safety(&filtered_levels, false) == Safe {
                        Some(*index)
                    } else {
                        None
                    }
                })
                .map(SafeByRemoving)
                .next()
                .unwrap_or(Unsafe)
        }
        _ => Unsafe,
    }
}

#[derive(Debug, PartialEq)]
enum Safety {
    Safe,
    Unsafe,
    SafeByRemoving(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_safety_part2_example() {
        assert_eq!(eval_safety(&[7, 6, 4, 2, 1], true), Safe);
        assert_eq!(eval_safety(&[1, 2, 7, 8, 9], true), Unsafe);
        assert_eq!(eval_safety(&[9, 7, 6, 2, 1], true), Unsafe);
        assert_eq!(eval_safety(&[1, 3, 2, 4, 5], true), SafeByRemoving(1));
        assert_eq!(eval_safety(&[8, 6, 4, 4, 1], true), SafeByRemoving(2));
        assert_eq!(eval_safety(&[1, 3, 6, 7, 9], true), Safe);
    }
}
