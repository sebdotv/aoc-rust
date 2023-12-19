use anyhow::Result;
use std::str::FromStr;

use crate::challenge::Day;
use crate::utils::grid::{Coord, Grid};

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (136, Some(109939)),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let grid: Grid<char> = Grid::from_str(data)?;
    let grid = slide_north(&grid);
    let sum = grid
        .iter()
        .filter_map(|(coord, cell)| {
            if cell == 'O' {
                let Coord(_, y) = coord;
                Some(grid.h - y)
            } else {
                None
            }
        })
        .sum::<usize>();
    Ok(sum)
}

fn part2(_data: &str) -> Result<usize> {
    todo!()
}

fn slide_north(grid: &Grid<char>) -> Grid<char> {
    let mut result = grid.transform(|(coord, &cell)| if cell == 'O' { '.' } else { cell });

    for (Coord(coord_x, coord_y), cell) in grid.iter() {
        if cell == 'O' {
            let mut y = coord_y;
            loop {
                if y == 0 || result.maybe_get(&Coord(coord_x, y - 1)) != Some(&'.') {
                    break;
                }
                y -= 1;
            }
            result.set(&Coord(coord_x, y), 'O');
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::trim_lines;

    #[test]
    fn test_part1_example() {
        let example = day().read_data_file("example").unwrap();
        let grid: Grid<char> = Grid::from_str(&example).unwrap();
        let actual = slide_north(&grid);
        let expected = r"
            OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....        
        ";
        assert_eq!(actual.to_string().trim(), trim_lines(expected));
    }
}
