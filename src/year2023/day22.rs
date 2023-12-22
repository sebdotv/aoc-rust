use anyhow::{bail, Result};
use itertools::Itertools;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::challenge::Day;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (5, None),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let bricks = data
        .lines()
        .map(Brick::from_str)
        .collect::<Result<Vec<_>>>()?;
    for brick in &bricks {
        println!("{:?}", brick);
    }
    Ok(5)
}

fn part2(_data: &str) -> Result<usize> {
    todo!()
}

#[derive(Debug)]
enum Brick {
    Cube {
        x: usize,
        y: usize,
        z: usize,
    },
    XLine {
        x: RangeInclusive<usize>,
        y: usize,
        z: usize,
    },
    YLine {
        x: usize,
        y: RangeInclusive<usize>,
        z: usize,
    },
    ZLine {
        x: usize,
        y: usize,
        z: RangeInclusive<usize>,
    },
}
impl FromStr for Brick {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').unwrap();
        let parse_xyz = |s: &str| {
            let (x, y, z) = s
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            (x, y, z)
        };
        let (start_x, start_y, start_z) = parse_xyz(start);
        let (end_x, end_y, end_z) = parse_xyz(end);
        let (len_x, len_y, len_z) = (end_x - start_x, end_y - start_y, end_z - start_z);
        let brick = match (len_x, len_y, len_z) {
            (0, 0, 0) => Brick::Cube {
                x: start_x,
                y: start_y,
                z: start_z,
            },
            (x, 0, 0) if x > 0 => Brick::XLine {
                x: start_x..=end_x,
                y: start_y,
                z: start_z,
            },
            (0, y, 0) if y > 0 => Brick::YLine {
                x: start_x,
                y: start_y..=end_y,
                z: start_z,
            },
            (0, 0, z) if z > 0 => Brick::ZLine {
                x: start_x,
                y: start_y,
                z: start_z..=end_z,
            },
            _ => bail!("Invalid brick: {}", s),
        };
        Ok(brick)
    }
}
