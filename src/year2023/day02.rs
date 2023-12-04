use std::ops::Add;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

use crate::challenge::Day;

pub fn day() -> Day<u32> {
    Day {
        part1_solutions: (8, Some(2416)),
        part2_solutions: Some((2286, Some(63307))),
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
    let sum = games
        .iter()
        .filter(|game| {
            game.reveals
                .iter()
                .all(|reveal| reveal.red <= 12 && reveal.green <= 13 && reveal.blue <= 14)
        })
        .map(|game| game.id)
        .sum();
    Ok(sum)
}

fn part2(data: &str) -> Result<u32> {
    let games = data
        .lines()
        .map(str::parse::<Game>)
        .collect::<Result<Vec<_>>>()?;

    let minimum_set = |game: &Game| -> SetOfCubes {
        let max = |f: &dyn Fn(&SetOfCubes) -> u32| game.reveals.iter().map(f).max().unwrap();
        let red = max(&|r: &SetOfCubes| r.red);
        let green = max(&|r: &SetOfCubes| r.green);
        let blue = max(&|r: &SetOfCubes| r.blue);
        SetOfCubes { red, green, blue }
    };

    let sum = games
        .iter()
        .map(minimum_set)
        .map(|set| set.red * set.green * set.blue)
        .sum();
    Ok(sum)
}

lazy_static! {
    static ref GAME_RE: Regex = Regex::new(r"^Game (\d+): (.*)$").unwrap();
}

#[derive(Debug)]
struct Game {
    id: u32,
    reveals: Vec<SetOfCubes>,
}
#[derive(Debug, Default)]
struct SetOfCubes {
    red: u32,
    green: u32,
    blue: u32,
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
        Ok(Game { id, reveals })
    }
}

impl FromStr for SetOfCubes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parse_u8 = |s: &str| {
            s.parse::<u32>()
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
                    other => Err(anyhow!("unknown color `{}`", other)),
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
