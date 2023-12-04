use std::ops::Range;

use anyhow::{Context, Result};
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::challenge::Day;

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

    let grid = Grid::from_lines(&lines);
    let symbols = grid
        .coords()
        .filter(|(x, y)| {
            let c = grid.get(*x, *y);
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
            x_range
                .clone()
                .any(|x| grid.neighbors(x, *y).iter().any(|p| symbols.contains(p)))
        })
        .map(get_number)
        .collect::<Result<Vec<_>>>()?;

    let sum = part_numbers.iter().sum();

    Ok(sum)
}

struct Grid<T> {
    w: usize,
    h: usize,
    data: Vec<T>,
}
impl Grid<char> {
    fn from_lines(lines: &[&str]) -> Self {
        let w = lines[0].len();
        let h = lines.len();
        let data = lines
            .iter()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>();
        Self { w, h, data }
    }
}

impl<T> Grid<T> {
    fn coords(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.w).cartesian_product(0..self.h)
    }

    fn get(&self, x: usize, y: usize) -> &T {
        self.data.get(x + y * self.w).unwrap()
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        (-1..=1isize)
            .flat_map(|dx| {
                (-1..=1isize).filter_map(move |dy| {
                    if dx != 0 || dy != 0 {
                        let (x, y) = (
                            isize::try_from(x).unwrap() + dx,
                            isize::try_from(y).unwrap() + dy,
                        );
                        if x >= 0
                            && y >= 0
                            && x < isize::try_from(self.w).unwrap()
                            && y < isize::try_from(self.h).unwrap()
                        {
                            Some((usize::try_from(x).unwrap(), usize::try_from(y).unwrap()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new(r"\d+").unwrap();
}

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

    let grid = Grid::from_lines(&lines);

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
                    let neighbors = grid.neighbors(x, *y);
                    neighbors
                        .iter()
                        .filter(|(x, y)| *grid.get(*x, *y) == '*')
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
