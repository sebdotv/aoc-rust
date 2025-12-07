use anyhow::Result;
use itertools::Itertools;

use crate::challenge::Day;
use crate::utils::grid::{Coord, Grid};

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (405, Some(30535)),
        part2_solutions: Some((400, Some(30844))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let grids = parse(data)?;
    Ok(summarize(&grids, 0))
}

fn part2(data: &str) -> Result<usize> {
    let grids = parse(data)?;
    Ok(summarize(&grids, 1))
}

fn summarize(grids: &[Pattern], diff: usize) -> usize {
    grids
        .iter()
        .map(|grid| {
            find_reflection_vertical(grid, diff)
                .map(|i| i + 1)
                .or(find_reflection_horizontal(grid, diff)
                    .map(|i| i + 1)
                    .map(|i| i * 100))
                .unwrap()
        })
        .sum()
}

fn parse(data: &str) -> Result<Vec<Pattern>> {
    let lines = data.lines().collect_vec();
    let patterns = lines.split(|line| line.is_empty()).collect_vec();
    patterns
        .iter()
        .map(|pattern| Pattern::from_lines(pattern))
        .collect::<Result<Vec<_>>>()
}

type Pattern = Grid<char>;

fn find_reflection_vertical(grid: &Pattern, diff: usize) -> Option<usize> {
    (0..grid.w - 1).find(|&col_before| test_reflection_vertical(grid, col_before) == diff)
}
fn find_reflection_horizontal(grid: &Pattern, diff: usize) -> Option<usize> {
    (0..grid.h - 1).find(|&row_before| test_reflection_horizontal(grid, row_before) == diff)
}

fn test_reflection_vertical(grid: &Pattern, col_before: usize) -> usize {
    let mut diff = 0;
    for y in 0..grid.h {
        for i in 0..grid.w.div_ceil(2) {
            if i > col_before {
                continue;
            }
            let (x1, x2) = (col_before - i, col_before + 1 + i);
            let (c1, c2) = (grid.maybe_get(&Coord(x1, y)), grid.maybe_get(&Coord(x2, y)));
            if compare_chars(c1, c2) == Some(false) {
                diff += 1;
            }
        }
    }
    diff
}

fn test_reflection_horizontal(grid: &Pattern, row_before: usize) -> usize {
    let mut diff = 0;
    for x in 0..grid.w {
        for i in 0..grid.h.div_ceil(2) {
            if i > row_before {
                continue;
            }
            let (y1, y2) = (row_before - i, row_before + 1 + i);
            let (c1, c2) = (grid.maybe_get(&Coord(x, y1)), grid.maybe_get(&Coord(x, y2)));
            if compare_chars(c1, c2) == Some(false) {
                diff += 1;
            }
        }
    }
    diff
}

fn compare_chars(c1: Option<&char>, c2: Option<&char>) -> Option<bool> {
    match (c1, c2) {
        (Some(c1), Some(c2)) => Some(c1 == c2),
        (None, None) => unreachable!(),
        _ => None,
    }
}
