use std::ops::{AddAssign, Mul};
use std::str::FromStr;

use anyhow::{bail, Result};
use itertools::Itertools;

use crate::challenge::Day;
use crate::utils::grid::Direction;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (62, Some(48503)),
        part2_solutions: Some((952408144115, Some(148442153147147))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let plan: DigPlan = data.parse()?;

    Ok(solve(plan.steps))
}

fn part2(data: &str) -> Result<usize> {
    let plan: DigPlan = data.parse()?;

    let steps = plan
        .steps
        .iter()
        .map(|step| {
            let (len, dir) = step.color.split_at(5);
            let dir = match dir {
                "0" => Direction::E,
                "1" => Direction::S,
                "2" => Direction::W,
                "3" => Direction::N,
                _ => bail!("Invalid direction"),
            };
            let len = usize::from_str_radix(len, 16)?;
            Ok(Step {
                dir,
                len,
                color: String::new(),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(solve(steps))
}

fn solve(steps: Vec<Step>) -> usize {
    assert_eq!(steps.first().unwrap().dir, Direction::E);
    assert_eq!(steps.last().unwrap().dir, Direction::N);

    let start = Coord2 { x: 0, y: 0 };
    let mut coords: Vec<Coord2> = Vec::new();
    let mut pos = start;
    for step in steps {
        coords.push(pos);
        let v = match step.dir {
            Direction::N => Coord2 { x: 0, y: 1 },
            Direction::S => Coord2 { x: 0, y: -1 },
            Direction::W => Coord2 { x: -1, y: 0 },
            Direction::E => Coord2 { x: 1, y: 0 },
        } * isize::try_from(step.len).unwrap();
        pos += v;
    }
    assert_eq!(pos, start);

    trench_area(&coords)
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

#[derive(Debug)]
struct Step {
    dir: Direction,
    len: usize,
    color: String,
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
        let len = len.parse()?;
        let color = color.strip_prefix("(#").unwrap().strip_suffix(')').unwrap();
        let color = color.to_owned();
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

fn trench_area(coords: &[Coord2]) -> usize {
    // Area of a Convex Polygon (Shoelace formula) + add length of outer trench (border)
    // https://www.mathwords.com/a/area_convex_polygon.htm

    let mut area_double = 0;
    let mut border = 0;
    coords
        .iter()
        .zip(coords.iter().cycle().skip(1))
        .for_each(|(c1, c2)| {
            area_double += c1.x * c2.y - c1.y * c2.x;
            border += (c2.x - c1.x).abs() + (c2.y - c1.y).abs();
        });
    usize::try_from(area_double.abs() + border).unwrap() / 2 + 1
}
