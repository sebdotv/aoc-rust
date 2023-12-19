use anyhow::Result;
use itertools::Itertools;

use crate::challenge::Day;
use crate::utils::grid::{Coord, Grid};

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (405, Some(30535)),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let lines = data.lines().collect_vec();
    let patterns = lines.split(|line| line.is_empty()).collect_vec();
    let grids = patterns
        .iter()
        .map(|pattern| Pattern::from_lines(pattern))
        .collect::<Result<Vec<_>>>()?;

    let sum = grids
        .iter()
        .map(|grid| {
            find_reflection_vertical(grid)
                .map(|i| i + 1)
                .or(find_reflection_horizontal(grid)
                    .map(|i| i + 1)
                    .map(|i| i * 100))
                .unwrap()
        })
        .sum();

    Ok(sum)
}

fn part2(_data: &str) -> Result<usize> {
    todo!()
}

type Pattern = Grid<char>;

fn find_reflection_vertical(grid: &Pattern) -> Option<usize> {
    (0..grid.w - 1).find(|&col_before| is_reflection_vertical(grid, col_before))
}
fn find_reflection_horizontal(grid: &Pattern) -> Option<usize> {
    (0..grid.h - 1).find(|&row_before| is_reflection_horizontal(grid, row_before))
}

fn is_reflection_vertical(grid: &Pattern, col_before: usize) -> bool {
    for y in 0..grid.h {
        for i in 0..((grid.w + 1) / 2) {
            if i > col_before {
                continue;
            }
            let (x1, x2) = (col_before - i, col_before + 1 + i);
            let (c1, c2) = (grid.maybe_get(&Coord(x1, y)), grid.maybe_get(&Coord(x2, y)));
            if !compare_chars(c1, c2) {
                return false;
            }
        }
    }
    true
}

fn is_reflection_horizontal(grid: &Pattern, row_before: usize) -> bool {
    for x in 0..grid.w {
        for i in 0..((grid.h + 1) / 2) {
            if i > row_before {
                continue;
            }
            let (y1, y2) = (row_before - i, row_before + 1 + i);
            let (c1, c2) = (grid.maybe_get(&Coord(x, y1)), grid.maybe_get(&Coord(x, y2)));
            if !compare_chars(c1, c2) {
                return false;
            }
        }
    }
    true
}

fn compare_chars(c1: Option<&char>, c2: Option<&char>) -> bool {
    match (c1, c2) {
        (Some(c1), Some(c2)) if c1 != c2 => false,
        (None, None) => unreachable!(),
        _ => true,
    }
}
