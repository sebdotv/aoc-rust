use std::str::FromStr;

use anyhow::Result;
use indexmap::map::Entry;
use indexmap::IndexMap;
use itertools::Itertools;

use crate::challenge::Day;
use crate::utils::grid::{Coord, Direction, Grid};

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (136, Some(109939)),
        part2_solutions: Some((64, Some(101010))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let grid: Grid<char> = Grid::from_str(data)?;
    let grid = slide(&grid, Direction::N);
    Ok(total_load(&grid))
}

fn total_load(grid: &Grid<char>) -> usize {
    grid.iter()
        .filter_map(|(coord, cell)| {
            if cell == 'O' {
                let Coord(_, y) = coord;
                Some(grid.h - y)
            } else {
                None
            }
        })
        .sum::<usize>()
}

fn part2(data: &str) -> Result<usize> {
    let mut grid: Grid<char> = Grid::from_str(data)?;
    let mut seen: IndexMap<String, usize> = IndexMap::new();
    let mut cycle = 0;
    loop {
        match seen.entry(grid.to_string()) {
            Entry::Occupied(entry) => {
                let period = cycle - entry.get();
                let remaining_cycles = 1_000_000_000 - cycle;
                let remaining_cycles = remaining_cycles % period;
                for _ in 0..remaining_cycles {
                    grid = slide_cycle(&grid);
                }
                break;
            }
            Entry::Vacant(entry) => {
                entry.insert(cycle);
            }
        }
        grid = slide_cycle(&grid);
        cycle += 1;
    }
    Ok(total_load(&grid))
}

fn slide(grid: &Grid<char>, direction: Direction) -> Grid<char> {
    let mut result = grid.transform(|(_, &cell)| if cell == 'O' { '.' } else { cell });

    let ((dx, dy), coords) = match direction {
        Direction::N => ((0, -1isize), grid.row_coords()),
        Direction::S => ((0, 1), grid.row_coords().into_iter().rev().collect_vec()),
        Direction::E => ((1, 0), grid.col_coords().into_iter().rev().collect_vec()),
        Direction::W => ((-1isize, 0), grid.col_coords()),
    };

    for coord_line in coords {
        for coord in coord_line {
            let cell = grid.get(&coord);
            let Coord(coord_x, coord_y) = coord;
            if *cell == 'O' {
                let mut x = coord_x;
                let mut y = coord_y;
                loop {
                    if (dx < 0 && x == 0)
                        || (dy < 0 && y == 0)
                        || (dx > 0 && x == grid.w - 1)
                        || (dy > 0 && y == grid.h - 1)
                    {
                        break;
                    }
                    let next_x = (isize::try_from(x).unwrap() + dx).try_into().unwrap();
                    let next_y = (isize::try_from(y).unwrap() + dy).try_into().unwrap();
                    if result.get(&Coord(next_x, next_y)) != &'.' {
                        break;
                    }
                    x = next_x;
                    y = next_y;
                }
                result.set(&Coord(x, y), 'O');
            }
        }
    }

    result
}

fn slide_cycle(grid: &Grid<char>) -> Grid<char> {
    let grid = slide(grid, Direction::N);
    let grid = slide(&grid, Direction::W);
    let grid = slide(&grid, Direction::S);
    slide(&grid, Direction::E)
}

#[cfg(test)]
mod tests {
    use crate::testing::trim_lines;

    use super::*;

    #[test]
    fn test_part1_example() {
        let example = day().read_data_file("example").unwrap();
        let grid: Grid<char> = Grid::from_str(&example).unwrap();
        let actual = slide(&grid, Direction::N);
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

    #[test]
    fn test_part2_example() {
        let example = day().read_data_file("example").unwrap();
        let grid: Grid<char> = Grid::from_str(&example).unwrap();

        // after 1 cycle
        let grid = slide_cycle(&grid);
        let expected = r"
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....    
        ";
        assert_eq!(grid.to_string().trim(), trim_lines(expected));

        // after 2 cycles
        let grid = slide_cycle(&grid);
        let expected = r"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #..OO###..
            #.OOO#...O
        ";
        assert_eq!(grid.to_string().trim(), trim_lines(expected));

        // after 3 cycles
        let grid = slide_cycle(&grid);
        let expected = r"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O
        ";
        assert_eq!(grid.to_string().trim(), trim_lines(expected));
    }
}
