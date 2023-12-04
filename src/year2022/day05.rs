use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use strum_macros::Display;

use crate::challenge::ChallengeDay;

pub fn day() -> ChallengeDay<String> {
    ChallengeDay {
        part1_solutions: ("CMZ".to_owned(), Some("MQTPGLLDN".to_owned())),
        part2_solutions: Some(("MCD".to_owned(), Some("LVZPSTTCZ".to_owned()))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    pub fn apply_part1(&mut self, op: &Move) {
        (0..op.n).for_each(|_| {
            let c = self.stacks[op.from - 1].pop().unwrap();
            self.stacks[op.to - 1].push(c);
        });
    }
    pub fn apply_part2(&mut self, op: &Move) {
        let from = &mut self.stacks[op.from - 1];
        let tail = from.split_off(from.len() - op.n);
        self.stacks[op.to - 1].extend(tail);
    }

    pub fn top(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect()
    }
}

impl Stacks {
    fn from_lines(lines: &[&str]) -> Self {
        let lines_iter = || lines.iter().dropping_back(1);
        let max_line_len = lines_iter().map(|line| line.len()).max().unwrap();
        let n = (max_line_len + 1) / 4;
        assert_eq!((max_line_len + 1) % 4, 0);
        let stacks = (0..n)
            .map(|i| {
                lines_iter()
                    .rev()
                    .filter_map(|line| line.chars().nth(i * 4 + 1))
                    .filter(|&c| c != ' ')
                    .collect()
            })
            .collect();
        Self { stacks }
    }
}

fn part1(data: &str) -> Result<String> {
    let (mut stacks, moves) = parse(data)?;

    moves.iter().for_each(|op| {
        stacks.apply_part1(op);
    });

    Ok(stacks.top())
}

fn parse(data: &str) -> Result<(Stacks, Vec<Move>)> {
    let lines = data.lines().collect_vec();

    let (start, moves) = lines
        .split(|line| line.is_empty())
        .collect_tuple()
        .ok_or(anyhow!("Could not split"))?;

    let stacks = Stacks::from_lines(start);

    let moves = moves
        .iter()
        .map(|line| {
            line.parse::<Move>()
                .with_context(|| format!("Could not parse {}", line))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((stacks, moves))
}

fn part2(data: &str) -> Result<String> {
    let (mut stacks, moves) = parse(data)?;

    moves.iter().for_each(|op| {
        stacks.apply_part2(op);
    });

    Ok(stacks.top())
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    n: usize,
    from: usize,
    to: usize,
}

lazy_static! {
    static ref MOVE_RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
}

#[derive(Display, Debug, PartialEq, Eq, thiserror::Error)]
enum ParseMoveError {
    InvalidMove,
    ParseIntError,
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = MOVE_RE.captures(s).ok_or(ParseMoveError::InvalidMove)?;
        let (_, [n, from, to]) = caps.extract();
        let parse_usize = |s: &str| {
            s.parse::<usize>()
                .map_err(|_| ParseMoveError::ParseIntError)
        };
        Ok(Move {
            n: parse_usize(n)?,
            from: parse_usize(from)?,
            to: parse_usize(to)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use ParseMoveError::*;

    use super::*;

    #[test]
    fn move_parser_works() {
        assert_eq!(parse_move("move 1 from 2 to 3"), Ok(new_move(1, 2, 3)));
        assert_eq!(parse_move("___move 1 from 2 to 3"), Err(InvalidMove));
        assert_eq!(parse_move("move 1 from 2 to 3___"), Err(InvalidMove));
        assert_eq!(parse_move("move 1000000 from 2 to 3"), Err(ParseIntError));
    }

    fn parse_move(s: &str) -> std::result::Result<Move, ParseMoveError> {
        s.parse::<Move>()
    }

    fn new_move(n: usize, from: usize, to: usize) -> Move {
        Move { n, from, to }
    }
}
