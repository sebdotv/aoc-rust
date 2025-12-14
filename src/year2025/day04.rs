use crate::challenge::Day;
use crate::utils::grid::{Coord, Grid};
use anyhow::Result;
use itertools::Itertools;
use strum_macros::EnumString;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (13, Some(1397)),
        part2_solutions: Some((43, Some(8758))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

#[derive(EnumString, Eq, PartialEq)]
enum Cell {
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "@")]
    Roll,
}

fn part1(data: &str) -> Result<usize> {
    let grid: Grid<Cell> = data.parse().unwrap();
    Ok(find_accessible(&grid).len())
}

fn find_accessible(grid: &Grid<Cell>) -> Vec<Coord> {
    grid.coords()
        .filter(|c| grid.get(c) == &Cell::Roll)
        .filter(|c| {
            let rolls = grid
                .neighbors_incl_diag(c)
                .iter()
                .filter(|c| grid.get(c) == &Cell::Roll)
                .count();
            rolls < 4
        })
        .collect_vec()
}

fn part2(data: &str) -> Result<usize> {
    let mut grid: Grid<Cell> = data.parse().unwrap();

    let mut removed = 0;
    loop {
        let accessible = find_accessible(&grid);
        if accessible.is_empty() {
            break;
        }

        for c in &accessible {
            grid.set(c, Cell::Empty);
        }

        removed += accessible.len();
    }

    Ok(removed)
}
