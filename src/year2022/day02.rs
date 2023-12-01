use crate::challenge::ChallengeDay;
use anyhow::{anyhow, Result};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn day() -> ChallengeDay<u32> {
    ChallengeDay {
        part1_solutions: (15, Some(11386)),
        part2_solutions: Some((12, Some(13600))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<u32> {
    let parse = |line: &str| {
        use Shape::*;
        let (shape, beats) = line.split_once(' ').ok_or(anyhow!("invalid line"))?;
        let opponent = match shape {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => return Err(anyhow!("invalid opponent move")),
        };
        let own = match beats {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => return Err(anyhow!("invalid own move")),
        };
        Ok((opponent, own))
    };
    let game = Game {
        moves: data.lines().map(parse).collect::<Result<_>>()?,
    };
    Ok(game.score())
}

fn part2(data: &str) -> Result<u32> {
    let parse = |line: &str| {
        use Outcome::*;
        use Shape::*;
        let (shape, beats) = line.split_once(' ').ok_or(anyhow!("invalid line"))?;
        let opponent = match shape {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => return Err(anyhow!("invalid opponent move")),
        };
        let outcome = match beats {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => return Err(anyhow!("invalid outcome")),
        };
        let own = Shape::iter()
            .find(|own| own.outcome_vs(&opponent) == outcome)
            .ok_or(anyhow!("could not find own move"))?;
        Ok((opponent, own))
    };
    let game = Game {
        moves: data.lines().map(parse).collect::<Result<_>>()?,
    };
    Ok(game.score())
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone, EnumIter)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(Debug)]
struct Round {
    own: Shape,
    outcome: Outcome,
}

impl Round {
    fn score(&self) -> u32 {
        use Outcome::*;
        use Shape::*;
        (match self.own {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }) + (match self.outcome {
            Loss => 0,
            Draw => 3,
            Win => 6,
        })
    }
}

impl Shape {
    fn outcome_vs(&self, other: &Shape) -> Outcome {
        use Outcome::*;
        use Shape::*;
        match (self, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Win,
            (a, b) if a == b => Draw,
            _ => Loss,
        }
    }
}

#[derive(Debug)]
struct Game {
    moves: Vec<(Shape, Shape)>,
}

impl Game {
    fn play(&self) -> Vec<Round> {
        self.moves
            .iter()
            .map(|(opponent, own)| Round {
                own: *own,
                outcome: own.outcome_vs(opponent),
            })
            .collect()
    }

    fn score(&self) -> u32 {
        self.play().iter().map(|round| round.score()).sum()
    }
}
