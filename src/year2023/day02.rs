use std::ops::Add;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

use crate::challenge::ChallengeDay;

pub fn day() -> ChallengeDay<u32> {
    ChallengeDay {
        part1_solutions: (8, Some(2416)),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<u32> {
    let games = data
        .lines()
        .map(str::parse::<Game>)
        .collect::<Result<Vec<_>>>()?;
    for game in &games {
        println!("{:?}", game);
    }
    let sum = games
        .iter()
        .filter(|game| {
            game.reveals
                .iter()
                .all(|reveal| reveal.red <= 12 && reveal.green <= 13 && reveal.blue <= 14)
        })
        .map(|game| game._id)
        .sum();
    Ok(sum)
}

fn part2(_data: &str) -> Result<u32> {
    todo!()
}

lazy_static! {
    static ref GAME_RE: Regex = Regex::new(r"^Game (\d+): (.*)$").unwrap();
    // static ref SET_RE: Regex =
    //     Regex::new(r"^((?P<red>\d+) red)?(, ?)((?P<green>\d+) green)?((, ?)(?P<blue>\d+) blue)?$")
    //         .unwrap();
}

#[derive(Debug)]
struct Game {
    _id: u32,
    reveals: Vec<SetOfCubes>,
}
#[derive(Debug, Default)]
struct SetOfCubes {
    red: u8,
    green: u8,
    blue: u8,
}
impl Add for SetOfCubes {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}
impl Game {}
impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let caps = GAME_RE
            .captures(s)
            .ok_or(anyhow!("invalid line: `{}`", s))?;
        let (_, [id, reveals]) = caps.extract();
        let parse_u32 = |s: &str| {
            s.parse::<u32>()
                .map_err(|_| anyhow!("number parse error for `{}`", s))
        };
        let id = parse_u32(id)?;
        let reveals = reveals
            .split("; ")
            .map(str::parse::<SetOfCubes>)
            .collect::<Result<Vec<_>>>()?;
        Ok(Game { _id: id, reveals })
    }
}

impl FromStr for SetOfCubes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parse_u8 = |s: &str| {
            s.parse::<u8>()
                .map_err(|_| anyhow!("number parse error for `{}`", s))
        };
        let sets = s
            .split(", ")
            .map(|s| {
                let (n, color) = s.split_once(' ').unwrap();
                let n = parse_u8(n)?;
                match color {
                    "red" => Ok(SetOfCubes {
                        red: n,
                        ..Default::default()
                    }),
                    "green" => Ok(SetOfCubes {
                        green: n,
                        ..Default::default()
                    }),
                    "blue" => Ok(SetOfCubes {
                        blue: n,
                        ..Default::default()
                    }),
                    other => Err(anyhow!("unknown color `{}`", color)),
                }
            })
            .collect::<Result<Vec<_>>>()?;
        let mut sum = SetOfCubes::default();
        for set in sets {
            sum = sum + set;
        }
        Ok(sum)
    }
}
