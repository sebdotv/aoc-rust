use anyhow::Result;
use indexmap::IndexSet;
use itertools::Itertools;
use strum_macros::{EnumIter, EnumString};

use crate::challenge::Day;
use crate::grid::{Coord, Grid};

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (374, Some(9550717)),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let grid: Grid<Cell> = Grid::from_str(data)?;
    let grid = expand(&grid);

    let galaxies = grid
        .iter()
        .filter_map(|(coord, cell)| (cell == Cell::Galaxy).then_some(coord))
        .collect_vec();

    let sum = galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.manhattan_distance(b))
        .sum();

    Ok(sum)
}

fn expand(grid: &Grid<Cell>) -> Grid<Cell> {
    let empty_rows: IndexSet<_> = (0..grid.h)
        .filter(|&y| (0..grid.w).all(|x| grid.get(&Coord(x, y)) == &Cell::Empty))
        .collect();
    let empty_cols: IndexSet<_> = (0..grid.w)
        .filter(|&x| (0..grid.h).all(|y| grid.get(&Coord(x, y)) == &Cell::Empty))
        .collect();
    let expanded = (0..grid.h)
        .flat_map(|y| {
            let row = (0..grid.w)
                .flat_map(|x| {
                    let cell = grid.get(&Coord(x, y));
                    if empty_cols.contains(&x) {
                        vec![*cell, *cell]
                    } else {
                        vec![*cell]
                    }
                })
                .collect_vec();
            if empty_rows.contains(&y) {
                vec![row.clone(), row]
            } else {
                vec![row]
            }
        })
        .collect_vec();
    Grid::from_data(expanded)
}

fn part2(_data: &str) -> Result<usize> {
    todo!()
}

#[derive(EnumString, Debug, Eq, PartialEq, strum_macros::Display, Copy, Clone, EnumIter)]
enum Cell {
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "#")]
    Galaxy,
}
