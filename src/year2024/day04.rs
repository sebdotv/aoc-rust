use crate::challenge::Day;
use crate::utils::grid::Direction::{E, N, S, W};
use crate::utils::grid::DirectionDiag::{NE, NW, SE, SW};
use crate::utils::grid::{Coord, Direction, DirectionDiag, Grid};
use anyhow::Result;
use itertools::Itertools;
use nonempty::{nonempty, NonEmpty};
use strum::IntoEnumIterator;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (18, Some(2567)),
        part2_solutions: Some((9, Some(2029))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: true,
    }
}

#[derive(Debug)]
enum Dir {
    Std(Direction),
    Diag(DirectionDiag),
}

fn part1(data: &str) -> Result<usize> {
    let grid: Grid<char> = data.parse()?;

    let dirs = Direction::iter()
        .map(Dir::Std)
        .chain(DirectionDiag::iter().map(Dir::Diag))
        .collect_vec();

    let pattern = nonempty!['X', 'M', 'A', 'S'];

    let mut count = 0;
    for (coord, _) in &grid {
        for dir in &dirs {
            if find_pattern(dir, &grid, &coord, &pattern) {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn part2(data: &str) -> Result<usize> {
    let grid: Grid<char> = data.parse()?;

    let diags = vec![SE, NW];

    let pattern = nonempty!['M', 'A', 'S'];

    let mut count = 0;
    for (coord, _) in &grid {
        for diag in &diags {
            // find MAS in SE/NW
            if find_pattern(&Dir::Diag(*diag), &grid, &coord, &pattern) {
                // find X-corresponding MAS
                let candidates = match diag {
                    SE => vec![([S, S], NE), ([E, E], SW)],
                    NW => vec![([N, N], SW), ([W, W], NE)],
                    _ => unreachable!(),
                };
                for (movement, other_diag) in candidates {
                    if let Some(true) = grid
                        .walk_multiple(&coord, &movement)
                        .map(|coord| find_pattern(&Dir::Diag(other_diag), &grid, &coord, &pattern))
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}

fn find_pattern(dir: &Dir, grid: &Grid<char>, coord: &Coord, remaining: &NonEmpty<char>) -> bool {
    if *grid.get(coord) == remaining.head {
        if let Some(tail) = NonEmpty::from_slice(&remaining.tail) {
            let coord = match dir {
                Dir::Std(d) => grid.walk(coord, *d),
                Dir::Diag(d) => grid.walk_diag(coord, *d),
            };
            if let Some(coord) = coord {
                find_pattern(dir, grid, &coord, &tail)
            } else {
                false // out of bounds
            }
        } else {
            true // match found
        }
    } else {
        false // not a match
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::trim_lines;

    #[test]
    fn test_part1() {
        let data = r"
            ..X...
            .SAMX.
            .A..A.
            XMAS.S
            .X....
            ";
        assert_eq!(part1(&trim_lines(data)).unwrap(), 4);
    }
}
