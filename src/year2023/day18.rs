#![allow(clippy::items_after_statements)]
#![allow(clippy::match_same_arms)]

use anyhow::{bail, Result};
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::challenge::Day;
use crate::utils::grid::{Coord, Direction, Grid};

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (62, None),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let grid = to_grid(data)?;
    println!("{}", grid);

    let mut interior: IndexSet<Coord> = IndexSet::new();
    // let mut interior = 0;

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum State {
        Outside,
        TrenchToInside,
        Inside,
        TrenchToOutside,
    }
    use State::*;

    for row in grid.row_coords() {
        let mut state = State::Outside;
        for c in row {
            let cell = grid.get(&c);
            state = match (state, cell) {
                (Outside, Cell::Empty) => Outside,
                (Outside, Cell::InitialHole | Cell::ColoredHole(_)) => TrenchToInside,
                (TrenchToInside, Cell::Empty) => Inside,
                (TrenchToInside, Cell::InitialHole | Cell::ColoredHole(_)) => TrenchToInside,
                (Inside, Cell::Empty) => Inside,
                (Inside, Cell::InitialHole | Cell::ColoredHole(_)) => TrenchToOutside,
                (TrenchToOutside, Cell::Empty) => Outside,
                (TrenchToOutside, Cell::InitialHole | Cell::ColoredHole(_)) => TrenchToOutside,
            };
            // println!("{:?}: {:?} -> {:?}", c, cell, state);
            if state == Inside {
                interior.insert(c);
                // interior += 1;
            }
        }
    }

    let grid = grid.transform(|(c, cell)| {
        if interior.contains(c) {
            Cell::ColoredHole([0, 0, 0])
        } else {
            *cell
        }
    });
    println!("{}", grid);

    Ok(0)

    // Ok(grid.iter().filter(|(_, cell)| *cell != Cell::Empty).count() + interior)
}

fn part2(_data: &str) -> Result<usize> {
    todo!()
}

fn to_grid(data: &str) -> Result<Grid<Cell>> {
    let plan: DigPlan = data.parse()?;
    let mut grid: UnboundedGrid<Cell> = UnboundedGrid::new();
    let mut pos = Coord2 { x: 0, y: 0 };

    let prev = grid.data.insert(pos, Cell::InitialHole);
    assert!(prev.is_none());

    for step in plan.steps {
        let (dx, dy) = match step.dir {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
            Direction::E => (1, 0),
        };
        for _ in 0..step.len {
            pos = Coord2 {
                x: pos.x + dx,
                y: pos.y + dy,
            };
            let prev = grid.data.insert(pos, Cell::ColoredHole(step.color));
            if let Some(prev) = prev {
                println!("Overwriting {:?} with {:?}", prev, step);
            }
        }
    }

    let grid = grid.into_grid(Cell::Empty);
    Ok(grid)
}

struct DigPlan {
    steps: Vec<Step>,
}
impl FromStr for DigPlan {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps = s
            .lines()
            .map(str::parse::<Step>)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { steps })
    }
}
type Color = [u8; 3];

#[derive(Debug)]
struct Step {
    dir: Direction,
    len: u8,
    color: Color,
}
impl FromStr for Step {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, len, color) = s.split_ascii_whitespace().collect_tuple().unwrap();
        let dir = match dir {
            "U" => Direction::N,
            "D" => Direction::S,
            "L" => Direction::W,
            "R" => Direction::E,
            _ => bail!("Invalid direction"),
        };
        let len = len.parse::<u8>()?;
        let color = color.strip_prefix("(#").unwrap().strip_suffix(')').unwrap();
        let color = hex::decode(color)?.try_into().unwrap();
        Ok(Self { dir, len, color })
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Coord2 {
    x: i32,
    y: i32,
}

struct UnboundedGrid<T> {
    data: IndexMap<Coord2, T>,
}

impl<T> UnboundedGrid<T> {
    fn new() -> Self {
        Self {
            data: IndexMap::new(),
        }
    }
}

impl<T> UnboundedGrid<T>
where
    T: Copy,
{
    fn into_grid(self, empty_value: T) -> Grid<T> {
        let (min_x, max_x) = self
            .data
            .keys()
            .map(|c| c.x)
            .minmax()
            .into_option()
            .unwrap();
        let (min_y, max_y) = self
            .data
            .keys()
            .map(|c| c.y)
            .minmax()
            .into_option()
            .unwrap();

        let w: usize = (max_x - min_x + 1).try_into().unwrap();
        let h: usize = (max_y - min_y + 1).try_into().unwrap();
        let mut grid: Grid<T> = Grid::empty(w, h, empty_value);
        for (c, t) in self.data {
            let x: usize = (c.x - min_x).try_into().unwrap();
            let y: usize = (c.y - min_y).try_into().unwrap();
            grid.set(&Coord(x, y), t);
        }
        grid
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    InitialHole,
    ColoredHole(Color),
}
impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Cell::Empty => '.',
            Cell::InitialHole => '#',
            Cell::ColoredHole(_color) => '#',
        };
        write!(f, "{}", char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::trim_lines;

    #[test]
    fn test_trench() {
        let data = day().read_data_file("example").unwrap();
        let grid = to_grid(&data).unwrap();
        let expected = r"
            #######
            #.....#
            ###...#
            ..#...#
            ..#...#
            ###.###
            #...#..
            ##..###
            .#....#
            .######
        ";
        assert_eq!(grid.to_string().trim(), trim_lines(expected));
    }
}
