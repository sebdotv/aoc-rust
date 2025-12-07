use anyhow::{Context, Result};
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use regex::Regex;
use std::ops::Range;
use std::sync::LazyLock;

use crate::challenge::Day;
use crate::utils::grid::{Coord, Grid};

pub fn day() -> Day<u32> {
    Day {
        part1_solutions: (4361, Some(544433)),
        part2_solutions: Some((467835, Some(76314915))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<u32> {
    let lines = data.lines().collect_vec();

    let grid: Grid<char> = Grid::from_lines(&lines)?;
    let symbols = grid
        .coords()
        .filter(|coord| {
            let c = grid.get(coord);
            *c != '.' && !c.is_ascii_digit()
        })
        .collect::<IndexSet<_>>();

    let get_number = |loc: &NumberLocation| -> Result<u32> {
        let (x_range, y) = loc;
        lines[*y][x_range.clone()]
            .parse::<u32>()
            .with_context(|| format!("Could not parse number at {loc:?}"))
    };

    let part_numbers = find_numbers(&lines)
        .iter()
        .filter(|(x_range, y)| {
            x_range.clone().any(|x| {
                grid.neighbors_incl_diag(&Coord(x, *y))
                    .iter()
                    .any(|p| symbols.contains(p))
            })
        })
        .map(get_number)
        .collect::<Result<Vec<_>>>()?;

    let sum = part_numbers.iter().sum();

    Ok(sum)
}

static NUMBER_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

type NumberLocation = (Range<usize>, usize);
fn find_numbers(lines: &[&str]) -> Vec<NumberLocation> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            NUMBER_RE
                .find_iter(line)
                .map(|m| (m.range(), y))
                .collect_vec()
        })
        .collect()
}

fn part2(data: &str) -> Result<u32> {
    let lines = data.lines().collect_vec();

    let grid: Grid<char> = Grid::from_lines(&lines)?;

    let get_number = |loc: &NumberLocation| -> Result<u32> {
        let (x_range, y) = loc;
        lines[*y][x_range.clone()]
            .parse::<u32>()
            .with_context(|| format!("Could not parse number at {loc:?}"))
    };

    let numbers = find_numbers(&lines);

    let numbers_and_possible_gears = numbers
        .iter()
        .flat_map(|loc @ (x_range, y)| {
            // find all gears that are adjacent to this number
            x_range
                .clone()
                .flat_map(|x| {
                    let neighbors = grid.neighbors_incl_diag(&Coord(x, *y));
                    neighbors
                        .iter()
                        .filter(|coord| *grid.get(coord) == '*')
                        .copied()
                        .collect_vec()
                })
                .map(|gear| (loc, gear))
                .collect::<IndexSet<_>>()
        })
        .collect_vec();

    let mut gear_candidates = IndexMap::new();
    for (loc, gear) in numbers_and_possible_gears {
        gear_candidates
            .entry(gear)
            .or_insert_with(Vec::new)
            .push(loc);
    }
    let sum = gear_candidates
        .iter()
        .filter_map(|(_, locs)| locs.iter().collect_tuple::<(_, _)>())
        .map(|(loc1, loc2)| Ok(get_number(loc1)? * get_number(loc2)?))
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum();

    Ok(sum)
}
