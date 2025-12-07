use crate::challenge::Day;
use crate::utils::grid::{Direction, Grid};
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;
use strum_macros::EnumString;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (21, Some(1678)),
        part2_solutions: Some((40, None)),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    use Cell::*;
    use Direction::*;
    let grid: Grid<Cell> = data.parse()?;
    let (start, _) = grid.iter().find(|(_, cell)| *cell == Start).unwrap();

    let mut split_count = 0;
    let mut beams = HashSet::new();
    let mut new_beams = HashSet::new();

    beams.insert(start);

    while !beams.is_empty() {
        for beam in beams.iter().sorted() {
            if let Some(next) = grid.walk(beam, S) {
                match grid.get(&next) {
                    Empty => {
                        new_beams.insert(next);
                    }
                    Splitter => {
                        split_count += 1;
                        new_beams.insert(grid.walk(&next, W).unwrap());
                        new_beams.insert(grid.walk(&next, E).unwrap());
                    }
                    Start => panic!("unexpected cell"),
                }
            }
        }
        beams = new_beams;
        new_beams = HashSet::new();
    }

    Ok(split_count)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumString)]
enum Cell {
    #[strum(serialize = "S")]
    Start,
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "^")]
    Splitter,
}

fn part2(_data: &str) -> Result<usize> {
    todo!()
}
