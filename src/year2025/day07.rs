use crate::challenge::Day;
use crate::utils::grid::{Coord, Direction, Grid};
use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use strum_macros::EnumString;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (21, Some(1678)),
        part2_solutions: Some((40, Some(357525737893560))),
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

fn part2(data: &str) -> Result<usize> {
    use Cell::*;
    let grid: Grid<Cell> = data.parse()?;
    let (start, _) = grid.iter().find(|(_, cell)| *cell == Start).unwrap();

    // (x, timelines)
    let mut beams = vec![(start.x(), 1)];

    for y in 1..grid.h {
        let mut new_beams = Vec::new();
        for (x, timelines) in &beams {
            match grid.get(&Coord(*x, y)) {
                Empty => {
                    // continue down
                    new_beams.push((*x, *timelines));
                }
                Splitter => {
                    // split
                    new_beams.push((x - 1, *timelines));
                    new_beams.push((x + 1, *timelines));
                }
                Start => panic!("unexpected cell"),
            }
        }

        // combine beams at the same x position
        let mut combined_beams: HashMap<usize, usize> = HashMap::new();
        for (x, timelines) in new_beams {
            *combined_beams.entry(x).or_default() += timelines;
        }

        beams = combined_beams
            .into_iter()
            .sorted_by_key(|&(x, _)| x)
            .collect();
    }

    let timelines = beams.iter().map(|(_, timelines)| timelines).sum();

    Ok(timelines)
}
