use anyhow::Result;
use indexmap::IndexSet;
use itertools::Itertools;
use strum_macros::{EnumIter, EnumString};

use crate::challenge::Day;
use crate::utils::grid::{Coord, Grid};

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (374, Some(9550717)),
        part2_solutions: Some((82000210, Some(648458253817))), // note: solution for example was not specified in the puzzle
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    solve(data, 2)
}

fn part2(data: &str) -> Result<usize> {
    solve(data, 1_000_000)
}

fn solve(data: &str, n: usize) -> Result<usize> {
    let grid: Grid<Cell> = data.parse()?;
    let extra_distance = n - 1;

    let empty_rows = find_empty_rows(&grid);
    let empty_cols = find_empty_cols(&grid);

    let galaxies = grid
        .iter()
        .filter_map(|(coord, cell)| (cell == Cell::Galaxy).then_some(coord))
        .collect_vec();

    let sum = galaxies
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| {
            let Coord(x1, y1) = *p1;
            let Coord(x2, y2) = *p2;

            let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
            let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };

            let mut x_dist = x2 - x1;
            let mut y_dist = y2 - y1;

            // there are fewer empty rows/cols than points between galaxies, so it's better to iterate over those
            for x in &empty_cols {
                if (x1..x2).contains(x) {
                    x_dist += extra_distance;
                }
            }
            for y in &empty_rows {
                if (y1..y2).contains(y) {
                    y_dist += extra_distance;
                }
            }

            x_dist + y_dist
        })
        .sum();

    Ok(sum)
}

fn find_empty_rows(grid: &Grid<Cell>) -> IndexSet<usize> {
    (0..grid.h)
        .filter(|&y| (0..grid.w).all(|x| grid.get(&Coord(x, y)) == &Cell::Empty))
        .collect()
}

fn find_empty_cols(grid: &Grid<Cell>) -> IndexSet<usize> {
    (0..grid.w)
        .filter(|&x| (0..grid.h).all(|y| grid.get(&Coord(x, y)) == &Cell::Empty))
        .collect()
}

#[derive(EnumString, Debug, Eq, PartialEq, strum_macros::Display, Copy, Clone, EnumIter)]
enum Cell {
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "#")]
    Galaxy,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_examples() {
        let example = day().read_data_file("example").unwrap();
        assert_eq!(solve(&example, 10).unwrap(), 1030);
        assert_eq!(solve(&example, 100).unwrap(), 8410);
    }
}
