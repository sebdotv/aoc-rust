use anyhow::Result;
use indexmap::IndexSet;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;

use crate::challenge::ChallengeDay;

pub fn day() -> ChallengeDay<u32> {
    ChallengeDay {
        part1_solutions: (4361, Some(544433)),
        part2_solutions: None,
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

    let part_numbers = find_numbers(&lines)
        .iter()
        .filter(|(x_range, y)| {
            x_range
                .clone()
                .any(|x| grid.neighbors(x, *y).iter().any(|p| symbols.contains(p)))
        })
        .map(|(x_range, y)| lines[*y][x_range.clone()].parse::<u32>().unwrap())
        .collect_vec();

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
                        let (x, y) = (x as isize + dx, y as isize + dy);
                        if x >= 0 && y >= 0 && x < self.w as isize && y < self.h as isize {
                            Some((x as usize, y as usize))
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

fn find_numbers(lines: &[&str]) -> Vec<(Range<usize>, usize)> {
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

fn part2(_data: &str) -> Result<u32> {
    todo!()
}
