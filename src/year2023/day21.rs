use anyhow::Result;
use itertools::Itertools;
use pathfinding::prelude::bfs_reach;
use polyfit_rs::polyfit_rs::polyfit;
use strum::IntoEnumIterator;

use crate::challenge::Day;
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
    if data.is_empty() {
        return Ok(0);
    }
    let grid: Grid<char> = data.parse()?;

    let n = 26501365;

    let (x, y): (Vec<f64>, Vec<f64>) = (0..3)
        .map(|i| {
            let steps = n % grid.w + i * grid.w;
            (steps as f64, part2_reach(&grid, steps) as f64)
        })
        .unzip();

    println!("x: {:?}", x);
    println!("y: {:?}", y);

    // let test_vectors = vec![(65, 3744), (196, 33417), (327, 92680)];
    //
    // let x = [65f64, 196f64, 327f64];
    // let y = [3744f64, 33417f64, 92680f64];
    let coeffs = polyfit(&x, &y, 2).unwrap();
    let (c, b, a) = coeffs.iter().collect_tuple().unwrap();

    let x = n as f64;
    let y = (a * x * x) + (b * x) + c;
    println!("y: {}", y);

    Ok(y as usize)
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

    bfs_reach(
        Pos {
            coord: (x as isize, y as isize),
            steps: 0,
        },
        successors,
    )
    .take_while(|pos| pos.steps <= steps + 10)
    .filter_map(|pos| (pos.steps == steps).then_some(pos.coord))
    .count()
}

#[cfg(test)]
mod tests {
    use crate::testing::trim_lines;
    use bigdecimal::num_bigint::BigInt;
    use bigdecimal::BigDecimal;
    use polyfit_rs::polyfit_rs::polyfit;

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

    #[test]
    fn test_part2_test_vectors() {
        let data = day().read_data_file("input").unwrap();
        let grid = data.parse().unwrap();

        let test_vectors = vec![(65, 3744), (196, 33417), (327, 92680)];
        for (steps, expected) in test_vectors {
            println!("steps: {}", steps);
            assert_eq!(part2_reach(&grid, steps), expected);
        }
    }
}
