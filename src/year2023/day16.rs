use anyhow::Result;
use indexmap::IndexSet;
use strum_macros::{EnumIter, EnumString};

use crate::challenge::Day;
use crate::utils::grid::{Coord, Direction, Grid};

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (46, Some(6605)),
        part2_solutions: Some((51, None)),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

const PART1_BEAM: Beam = Beam {
    coord: Coord(0, 0),
    dir: Direction::E,
};

fn part1(data: &str) -> Result<usize> {
    let grid: Grid<Tile> = data.parse()?;
    let mut solver = Solver::new(&grid, vec![PART1_BEAM]);
    solver.solve();
    Ok(solver.energized().len())
}

fn part2(data: &str) -> Result<usize> {
    let grid: Grid<Tile> = data.parse()?;

    let mut beams = vec![];
    for x in 0..grid.w {
        beams.push(Beam {
            coord: Coord(x, 0),
            dir: Direction::S,
        });
        beams.push(Beam {
            coord: Coord(x, grid.h - 1),
            dir: Direction::N,
        });
    }
    for y in 0..grid.h {
        beams.push(Beam {
            coord: Coord(0, y),
            dir: Direction::E,
        });
        beams.push(Beam {
            coord: Coord(grid.w - 1, y),
            dir: Direction::W,
        });
    }
    let max = beams
        .iter()
        .map(|beam| {
            let mut solver = Solver::new(&grid, vec![*beam]);
            solver.solve();
            solver.energized().len()
        })
        .max()
        .unwrap();
    Ok(max)
}

#[derive(EnumString, Debug, Eq, PartialEq, strum_macros::Display, Copy, Clone, EnumIter)]
enum Tile {
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "/")]
    MirrorNESW,
    #[strum(serialize = "\\")]
    MirrorNWSE,
    #[strum(serialize = "|")]
    SplitterV,
    #[strum(serialize = "-")]
    SplitterH,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Beam {
    coord: Coord,
    dir: Direction,
}
struct Solver<'a> {
    grid: &'a Grid<Tile>,
    visited: IndexSet<Beam>,
    beams: Vec<Beam>,
}

impl<'a> Solver<'a> {
    fn new(grid: &'a Grid<Tile>, beams: Vec<Beam>) -> Self {
        Self {
            grid,
            beams,
            visited: IndexSet::new(),
        }
    }

    fn solve_step(&mut self) -> bool {
        if let Some(beam) = self.beams.pop() {
            self.visited.insert(beam);
            let tile = self.grid.get(&beam.coord);
            let candidates: Vec<Direction> = match tile {
                Tile::Empty => {
                    vec![beam.dir]
                }
                Tile::MirrorNESW => {
                    let dir = match beam.dir {
                        Direction::N => Direction::E,
                        Direction::S => Direction::W,
                        Direction::E => Direction::N,
                        Direction::W => Direction::S,
                    };
                    vec![dir]
                }
                Tile::MirrorNWSE => {
                    let dir = match beam.dir {
                        Direction::N => Direction::W,
                        Direction::S => Direction::E,
                        Direction::E => Direction::S,
                        Direction::W => Direction::N,
                    };
                    vec![dir]
                }
                Tile::SplitterV => match beam.dir {
                    Direction::N | Direction::S => vec![beam.dir],
                    Direction::E | Direction::W => vec![Direction::N, Direction::S],
                },
                Tile::SplitterH => match beam.dir {
                    Direction::N | Direction::S => vec![Direction::E, Direction::W],
                    Direction::E | Direction::W => vec![beam.dir],
                },
            };
            for dir in candidates {
                if let Some(coord) = self.grid.walk(&beam.coord, dir) {
                    let beam = Beam { coord, dir };
                    if self.visited.contains(&beam) {
                        continue;
                    }
                    self.beams.push(beam);
                }
            }
            true
        } else {
            false
        }
    }

    fn solve(&mut self) {
        while self.solve_step() {}
    }

    #[allow(dead_code)]
    fn to_energized_grid(&self) -> Grid<String> {
        let energized = self.energized();
        self.grid.transform(|(coord, _tile)| {
            (if energized.contains(coord) { "#" } else { "." }).to_owned()
        })
    }
    fn energized(&self) -> IndexSet<Coord> {
        self.visited.iter().map(|beam| beam.coord).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::trim_lines;

    #[test]
    fn test_part1_example() {
        let data = day().read_data_file("example").unwrap();
        let grid: Grid<Tile> = data.parse().unwrap();
        let mut solver = Solver::new(&grid, vec![PART1_BEAM]);
        solver.solve();
        let energized = solver.to_energized_grid().to_string();
        let expected = r"
            ######....
            .#...#....
            .#...#####
            .#...##...
            .#...##...
            .#...##...
            .#..####..
            ########..
            .#######..
            .#...#.#..        
        ";
        assert_eq!(energized.trim(), trim_lines(expected));
    }
}
