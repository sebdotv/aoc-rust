use anyhow::Result;
use itertools::Itertools;

use crate::challenge::Day;
use crate::utils::grid::{Coord, Direction, Grid};
use pathfinding::prelude::bfs_reach;
use strum::IntoEnumIterator;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (0, Some(3660)),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    if data.is_empty() {
        return Ok(0);
    }
    let grid: Grid<char> = data.parse()?;
    let ends = possibilities(&grid, 64);
    Ok(ends.len())
}

fn part2(_data: &str) -> Result<usize> {
    todo!()
}

fn possibilities(grid: &Grid<char>, steps: usize) -> Vec<Coord> {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    struct Pos {
        coord: Coord,
        steps: usize,
    }

    let successors = |p: &Pos| {
        Direction::iter()
            .filter_map(|dir| grid.walk(&p.coord, dir))
            .filter(|c| grid.get(c) != &'#')
            .map(|c| Pos {
                coord: c,
                steps: p.steps + 1,
            })
            .collect_vec()
    };

    let (start,) = grid
        .iter()
        .filter_map(|(coord, c)| (c == 'S').then_some(coord))
        .collect_tuple()
        .unwrap();

    bfs_reach(
        Pos {
            coord: start,
            steps: 0,
        },
        successors,
    )
    .take_while(|pos| pos.steps <= steps)
    .filter_map(|pos| (pos.steps == steps).then_some(pos.coord))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::trim_lines;

    #[test]
    fn test_part1_example() -> Result<()> {
        let data = r"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........         
        ";
        let data = trim_lines(data);
        let grid = data.parse()?;

        // after 1 step
        let expected = r"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#O#....
            .##.OS####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........        
        ";
        assert_eq!(solution(&grid, 1).trim(), trim_lines(expected));

        // after 2 steps
        let expected = r"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#O..#..
            ....#.#....
            .##O.O####.
            .##.O#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........    
        ";
        assert_eq!(solution(&grid, 2).trim(), trim_lines(expected));

        // after 3 steps
        let expected = r"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#.O.#..
            ...O#O#....
            .##.OS####.
            .##O.#...#.
            ....O..##..
            .##.#.####.
            .##..##.##.
            ........... 
        ";
        assert_eq!(solution(&grid, 3).trim(), trim_lines(expected));

        // after 6 steps
        let expected = r"
            ...........
            .....###.#.
            .###.##.O#.
            .O#O#O.O#..
            O.O.#.#.O..
            .##O.O####.
            .##.O#O..#.
            .O.O.O.##..
            .##.#.####.
            .##O.##.##.
            ...........
        ";
        assert_eq!(solution(&grid, 6).trim(), trim_lines(expected));

        Ok(())
    }

    fn solution(grid: &Grid<char>, steps: usize) -> String {
        let ends = possibilities(grid, steps);
        grid.transform(|(coord, c)| if ends.contains(coord) { 'O' } else { *c })
            .to_string()
    }
}
