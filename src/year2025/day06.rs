use crate::challenge::Day;
use crate::utils::grid::{Coord, Grid};
use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use nom::AsChar;
use std::ops::Range;
use std::str::FromStr;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (4277556, Some(5227286044585)),
        part2_solutions: Some((3263827, Some(10227753257799))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let tokens = data
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .collect_vec();
    let (operators, numbers) = tokens.split_last().unwrap();
    let numbers = numbers
        .iter()
        .map(|line| {
            line.iter()
                .map(|num| num.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    // flip numbers
    let numbers: Vec<Vec<_>> = (0..numbers[0].len())
        .map(|i| numbers.iter().map(|line| line[i]).collect_vec())
        .collect_vec();

    let sum = numbers
        .iter()
        .zip(operators)
        .map(|(nums, op)| {
            let op: Operator = op.parse().unwrap();
            op.apply(nums)
        })
        .sum();
    Ok(sum)
}

fn part2(data: &str) -> Result<usize> {
    use Cell::*;

    // fill lines with trailing whitespaces
    let lines = data.lines().map(ToOwned::to_owned).collect_vec();
    let max_line_len = lines.iter().map(String::len).max().unwrap();
    let mut lines = lines;
    for line in &mut lines {
        let diff = max_line_len - line.len();
        if diff > 0 {
            *line = format!("{}{}", line, " ".repeat(diff));
        }
    }

    let grid: Grid<Cell> = Grid::from_lines(&lines)?;

    // walk the operator line from R to L
    let mut prev_op_x = None;
    let mut sum = 0;
    for x in (0..grid.w).rev() {
        let coord = Coord(x, grid.h - 1);
        match grid.get(&coord) {
            Operator(op) => {
                let end_x = prev_op_x.map_or(grid.w, |x| x - 1);
                let output = process_column(&grid, op, x..end_x);
                sum += output;
                prev_op_x = Some(x);
            }
            Whitespace => {}
            other => {
                bail!("unexpected cell: {:?}", other)
            }
        }
    }

    Ok(sum)
}

fn process_column(grid: &Grid<Cell>, op: &Operator, x_range: Range<usize>) -> usize {
    use Cell::*;

    // right to left
    let mut numbers = vec![];
    for x in x_range.rev() {
        // build number from top to bottom
        let mut number: String = String::new();
        for y in 0..grid.h - 1 {
            let coord = Coord(x, y);
            match grid.get(&coord) {
                Digit(d) => number.push_str(&d.to_string()),
                Whitespace => {}
                _ => panic!("unexpected cell"),
            };
        }
        let number = number.parse::<usize>().unwrap();
        numbers.push(number);
    }

    op.apply(&numbers)
}

#[derive(Debug)]
enum Cell {
    Digit(u8),
    Operator(Operator),
    Whitespace,
}
impl FromStr for Cell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == " " {
            return Ok(Self::Whitespace);
        }
        if let Ok(digit) = s.parse::<u8>() {
            return Ok(Self::Digit(digit));
        }
        if let Ok(op) = s.parse::<Operator>() {
            return Ok(Self::Operator(op));
        }
        Err(anyhow!("unknown cell type"))
    }
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Mul,
}
impl Operator {
    fn apply(&self, nums: &[usize]) -> usize {
        match self {
            Self::Add => nums.iter().sum::<usize>(),
            Self::Mul => nums.iter().product(),
            _ => panic!("unknown operator"),
        }
    }
}
impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(anyhow!("unknown operator")),
        }
    }
}
