use crate::challenge::Day;
use anyhow::Result;
use itertools::Itertools;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (1227775554, Some(30599400849)),
        part2_solutions: Some((4174379265, Some(46270373595))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    sum_of_invalid(data, is_invalid_part1)
}
fn part2(data: &str) -> Result<usize> {
    sum_of_invalid(data, is_invalid_part2)
}

fn is_invalid_part1(i: usize) -> bool {
    let s = i.to_string();
    let (a, b) = s.split_at(s.len() / 2);
    a == b
}
fn is_invalid_part2(i: usize) -> bool {
    let s = i.to_string();
    let chars = s.chars().collect_vec();
    for n in 1..s.len() {
        let mut it = chars.chunks(n);
        let first = it.next().unwrap();
        if it.all(|chunk| chunk == first) {
            return true;
        }
    }
    false
}

fn sum_of_invalid(data: &str, f: impl Fn(usize) -> bool) -> Result<usize> {
    Ok(data
        .trim_end()
        .split(',')
        .flat_map(|s| {
            let (first, last) = s.split_once('-').unwrap();
            let first: usize = first.parse().unwrap();
            let last: usize = last.parse().unwrap();
            first..=last
        })
        .filter(|i| f(*i))
        .sum())
}
