use anyhow::{bail, Result};
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, Mul};
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
    let plan: DigPlan = data.parse()?;

    for step in &plan.steps {
        println!("{:?}", step);
    }

    assert_eq!(plan.steps.first().unwrap().dir, Direction::E);
    assert_eq!(plan.steps.last().unwrap().dir, Direction::N);

    let start = Coord2 { x: 0, y: 0 };
    let mut coords: Vec<Coord2> = Vec::new();
    // coords.push(start);
    let mut pos = start;
    for step in plan.steps {
        coords.push(pos);
        let v = match step.dir {
            Direction::N => Coord2 { x: 0, y: 1 },
            Direction::S => Coord2 { x: 0, y: -1 },
            Direction::W => Coord2 { x: -1, y: 0 },
            Direction::E => Coord2 { x: 1, y: 0 },
        } * step.len as isize;
        pos += v;
    }
    assert_eq!(pos, start);
    // coords.push(pos);

    coords.reverse(); // we want counter-clockwise

    println!("{} coords", coords.len());
    for c in &coords {
        println!("{:?}", c);
    }

    // coords
    //     .iter()
    //     .zip(coords.iter().skip(1))
    //     .for_each(|(c1, c2)| {
    //         area_double += c1.x * c2.y - c1.y * c2.x;
    //         println!("{}", area_double);
    //     });

    // for i in 0..coords.len() {
    //     let j = (i + 1) % coords.len();
    //     let c1 = coords[i];
    //     let c2 = coords[j];
    //     area_double += c1.x * c2.y - c1.y * c2.x;
    //     println!("{}", area_double);
    // }

    let sum = trench_area(&coords);

    Ok(sum)
}
fn part1_old(data: &str) -> Result<usize> {
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

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord2 {
    x: isize,
    y: isize,
}
impl Mul<isize> for Coord2 {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl AddAssign for Coord2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
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

fn trench_area(coords: &[Coord2]) -> usize {
    // Area of a Convex Polygon (Shoelace formula) + add length of outer trench
    // https://www.mathwords.com/a/area_convex_polygon.htm

    let mut area_double = 0;
    let mut border = 0;
    coords
        .iter()
        .zip(coords.iter().cycle().skip(1))
        .for_each(|(c1, c2)| {
            println!("{:?} {:?}", c1, c2);
            area_double += c1.x * c2.y - c1.y * c2.x;
            border += (c2.x - c1.x).abs() + (c2.y - c1.y).abs();
        });
    (area_double.abs() + border) as usize / 2 + 1
}

fn area_old(coords: &[Coord2]) -> usize {
    // Area of a Convex Polygon
    // https://www.mathwords.com/a/area_convex_polygon.htm

    let mut area_double = 0;
    coords
        .iter()
        .zip(coords.iter().cycle().skip(1))
        .for_each(|(c1, c2)| {
            area_double += c1.x * c2.y - c2.x * c1.y;
        });
    (area_double.abs() / 2) as usize
}

fn area2_2(coords: &[Coord2]) -> usize {
    // Area of a non-convex polygon
    // https://en.wikipedia.org/wiki/Shoelace_formula#Triangle_formula

    let mut area2 = 0;

    for i in 0..coords.len() {
        let i_plus_1 = (i + 1) % coords.len();
        let i_minus_1 = ((i as isize - 1).rem_euclid(coords.len() as isize)) as usize;
        let c_i = coords[i];
        let c_i_plus_1 = coords[i_plus_1];
        let c_i_minus_1 = coords[i_minus_1];
        area2 += c_i.x * (c_i_plus_1.y - c_i_minus_1.y);
        println!("{}", area2);
    }

    // coords
    //     .iter()
    //     .zip(coords.iter().cycle().skip(1))
    //     .for_each(|(c1, c2)| {
    //         area_double += c1.x * c2.y - c2.x * c1.y;
    //     });
    area2.abs() as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::trim_lines;

    #[test]
    fn test_area_bad() {
        let coords = vec![(1, 5), (-4, 3), (5, 1), (2, 5)];
        let coords = coords
            .into_iter()
            .map(|(x, y)| Coord2 { x, y })
            .collect_vec();
        assert_eq!(trench_area(&coords), 30);
    }

    #[test]
    fn test_area() {
        let coords = vec![(1, 6), (3, 1), (7, 2), (4, 4), (8, 5)];
        let coords = coords
            .into_iter()
            .map(|(x, y)| Coord2 { x, y })
            .collect_vec();
        assert_eq!(trench_area(&coords), 33);
    }

    // #[test]
    // fn test_trench() {
    //     let data = day().read_data_file("example").unwrap();
    //     let grid = to_grid(&data).unwrap();
    //     let expected = r"
    //         #######
    //         #.....#
    //         ###...#
    //         ..#...#
    //         ..#...#
    //         ###.###
    //         #...#..
    //         ##..###
    //         .#....#
    //         .######
    //     ";
    //     assert_eq!(grid.to_string().trim(), trim_lines(expected));
    // }
}
