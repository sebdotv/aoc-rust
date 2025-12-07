use crate::challenge::Day;
use anyhow::Result;
use itertools::Itertools;
use std::ops::RangeInclusive;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (3, Some(664)),
        part2_solutions: Some((14, Some(350780324308385))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let (fresh_ids, lines_avail_ids) = parse_ranges(data);

    // each line in lines_avail_ids is a single id
    let mut fresh_count = 0;
    for line in lines_avail_ids {
        let id: usize = line.parse().unwrap();
        for range in &fresh_ids {
            if range.contains(&id) {
                fresh_count += 1;
                break;
            }
        }
    }

    Ok(fresh_count)
}

fn part2(data: &str) -> Result<usize> {
    let (fresh_ids, _) = parse_ranges(data);
    let fresh_ids = merge_intervals(&fresh_ids);
    let sum = fresh_ids
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum();
    Ok(sum)
}

fn parse_ranges(data: &str) -> (Vec<RangeInclusive<usize>>, Vec<&str>) {
    let lines = data.lines().collect_vec();

    // split on empty line
    let (lines_fresh_ids, lines_avail_ids) =
        lines.split(|line| line.is_empty()).collect_tuple().unwrap();

    let mut fresh_ids = Vec::new();

    // each line in lines_fresh_ids is an inclusive range of ids
    for line in lines_fresh_ids {
        let (start, end) = line.split_once('-').unwrap();
        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();
        fresh_ids.push(start..=end);
    }

    (fresh_ids, lines_avail_ids.to_vec())
}

fn merge_intervals(intervals: &[RangeInclusive<usize>]) -> Vec<RangeInclusive<usize>> {
    let mut result: Vec<RangeInclusive<usize>> = vec![];
    for range in intervals.iter().sorted_by_key(|r| r.start()) {
        if let Some(p) = result.last().cloned() {
            if range.start() > p.end() {
                result.push(range.clone());
            } else if range.end() <= p.end() {
                // skip
            } else {
                let merged = (*p.start())..=(*range.end());
                let prev = result.pop();
                assert_eq!(prev, Some(p));
                result.push(merged);
            }
        } else {
            result.push(range.clone());
        }
    }
    result
}
