use anyhow::Result;
use itertools::Itertools;
use pathfinding::prelude::bfs_reach;
use polyfit_rs::polyfit_rs::polyfit;
use strum::IntoEnumIterator;

use crate::challenge::Day;
use crate::utils::f64_conversions::{try_f64_from_usize, try_usize_from_f64};
use crate::utils::grid::{Coord, Direction, Grid};

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (0, Some(3660)),
        part2_solutions: Some((0, Some(605492675373144))),
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
    let ends = part1_reach(&grid, 64);
    Ok(ends.len())
}

fn part2(data: &str) -> Result<usize> {
    const STEPS: usize = 26501365;

    if data.is_empty() {
        return Ok(0);
    }
    let grid: Grid<char> = data.parse()?;

    let (xs, ys): (Vec<f64>, Vec<f64>) = (0..3)
        .map(|i| {
            let steps = STEPS % grid.w + i * grid.w;
            (
                try_f64_from_usize(i).unwrap(),
                try_f64_from_usize(part2_reach(&grid, steps)).unwrap(),
            )
        })
        .unzip();

    let coeffs = polyfit(&xs, &ys, 2).unwrap();

    let (coeff_c, coeff_b, coeff_a) = coeffs
        .iter()
        .map(|f| f.round())
        .map(|f| try_usize_from_f64(f).unwrap())
        .collect_tuple()
        .unwrap();

    let x = STEPS / grid.w; // ignore remainder
    let y = (coeff_a * x * x) + (coeff_b * x) + coeff_c;

    Ok(y)
}

fn part1_reach(grid: &Grid<char>, steps: usize) -> Vec<Coord> {
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

fn part2_reach(grid: &Grid<char>, steps: usize) -> usize {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    struct Pos {
        coord: (isize, isize),
        steps: usize,
    }

    let successors = |p: &Pos| {
        let (x, y) = p.coord;
        Direction::iter()
            .map(|dir| match dir {
                Direction::N => (x, y - 1),
                Direction::S => (x, y + 1),
                Direction::E => (x + 1, y),
                Direction::W => (x - 1, y),
            })
            .filter(|(x, y)| grid.get(&grid.map_virtual(*x, *y)) != &'#')
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

    let Coord(x, y) = start;
    let (x, y) = (isize::try_from(x).unwrap(), isize::try_from(y).unwrap());

    bfs_reach(
        Pos {
            coord: (x, y),
            steps: 0,
        },
        successors,
    )
    .take_while(|pos| pos.steps <= steps)
    .filter_map(|pos| (pos.steps == steps).then_some(pos.coord))
    .count()
}

#[cfg(test)]
mod tests {
    use crate::testing::trim_lines;

    use super::*;

    const EXAMPLE: &str = r"
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

    #[test]
    fn test_part1_example() -> Result<()> {
        let data = trim_lines(EXAMPLE);
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
        let ends = part1_reach(grid, steps);
        grid.transform(|(coord, c)| if ends.contains(coord) { 'O' } else { *c })
            .to_string()
    }
}
