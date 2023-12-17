use crate::challenge::Day;
use crate::utils::grid::{Coord, Direction, Grid, Turn};
use anyhow::Result;
use colored::Colorize;
use indexmap::IndexSet;
use itertools::Itertools;
use pathfinding::prelude::astar;
use strum::IntoEnumIterator;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (102, Some(1110)),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let grid: Grid<Cost> = data.parse()?;
    let puzzle = Puzzle { grid };
    let start = Pos {
        prev_action: None,
        coord: Coord(0, 0),
    };
    let goal_coord: Coord = puzzle.grid.bottom_right();
    let result = astar(
        &start,
        |p| puzzle.successors(p),
        |p| p.coord.manhattan_distance(&goal_coord),
        |p| p.coord == goal_coord,
    );
    let (_path, path_cost) = result.unwrap();
    Ok(path_cost)
}

#[allow(dead_code)]
fn dump_path(puzzle: &Puzzle, path: &[Pos]) {
    let path: IndexSet<_> = path.iter().map(|p| p.coord).collect();
    println!(
        "{}",
        puzzle.grid.transform(|(coord, cost)| {
            if path.contains(coord) {
                cost.to_string().bold().red().to_string()
            } else {
                cost.to_string()
            }
        })
    );
}

fn part2(_data: &str) -> Result<usize> {
    todo!()
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    prev_action: Option<(Direction, usize)>,
    coord: Coord,
}

enum Action {
    Rotate(Turn),
    Forward,
}

type Cost = usize;

struct Puzzle {
    grid: Grid<Cost>,
}

impl Puzzle {
    fn successors(&self, pos: &Pos) -> Vec<(Pos, Cost)> {
        use Action::*;
        let options = if let Some((dir, len_in_dir)) = pos.prev_action {
            let mut options = Turn::iter().map(Rotate).collect_vec();
            if len_in_dir < 3 {
                options.push(Forward);
            }
            options
                .into_iter()
                .filter_map(|action| {
                    let (dir, len_in_dir) = match action {
                        Rotate(turn) => (turn.apply(dir), 1),
                        Forward => (dir, len_in_dir + 1),
                    };
                    self.grid.walk(&pos.coord, dir).map(|coord| Pos {
                        prev_action: Some((dir, len_in_dir)),
                        coord,
                    })
                })
                .collect_vec()
        } else {
            Direction::iter()
                .filter_map(|dir| {
                    self.grid.walk(&pos.coord, dir).map(|coord| Pos {
                        prev_action: Some((dir, 1)),
                        coord,
                    })
                })
                .collect_vec()
        };

        options
            .into_iter()
            .map(|pos| {
                let cost = *self.grid.get(&pos.coord);
                (pos, cost)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::trim_lines;

    #[test]
    fn part1_works() {
        let data = r"
            12
            34
        ";
        assert_eq!(part1(&trim_lines(data)).unwrap(), 6);
        let data = r"
            113
            416
            711
        ";
        assert_eq!(part1(&trim_lines(data)).unwrap(), 4);
    }
}
