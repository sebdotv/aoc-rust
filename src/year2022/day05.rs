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
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn from_lines(lines: Vec<String>) -> Self {
        let max_line_len = lines
            .iter()
            .dropping_back(1)
            .map(|line| line.len())
            .max()
            .unwrap();
        let n = (max_line_len + 1) / 4;
        assert_eq!((max_line_len + 1) % 4, 0);
        let stacks = (0..n)
            .map(|i| {
                lines
                    .iter()
                    .rev()
                    .map(|line| line.chars().nth(i * 4 + 1).unwrap())
                    .collect()
            })
            .collect();
        Self { stacks }
    }
}

fn part1(data: &str) -> Result<String> {
    let lines = data.lines().collect_vec();
    let (start, moves) = lines
        .split(|line| line.is_empty())
        .collect_tuple()
        .ok_or(anyhow!("Could not split"))?;

    // todo fix this line:
    // Stacks::from_lines(start.iter().map(|s| s.to_owned()).collect());

    let moves = moves
        .iter()
        .map(|line| {
            line.parse::<Move>()
                .with_context(|| format!("Could not parse {}", line))
        })
        .collect::<Result<Vec<_>>>()?;

    // moves.iter().for_each(|op| {
    //     println!("{:?}", op);
    // });

    Ok("todo".to_owned())
}

fn part2(_data: &str) -> Result<String> {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    n: u8,
    from: u8,
    to: u8,
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
        let parse_u8 = |s: &str| s.parse::<u8>().map_err(|_| ParseMoveError::ParseIntError);
        Ok(Move {
            n: parse_u8(n)?,
            from: parse_u8(from)?,
            to: parse_u8(to)?,
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

    fn new_move(n: u8, from: u8, to: u8) -> Move {
        Move { n, from, to }
    }
}
