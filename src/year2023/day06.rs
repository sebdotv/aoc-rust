use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

use crate::challenge::Day;
use crate::f64_utils::{try_f64_from_u64, try_u64_from_f64};

pub fn day() -> Day<u64> {
    Day {
        part1_solutions: (288, Some(800280)),
        part2_solutions: Some((71503, Some(45128024))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

struct Puzzle {
    races: Vec<Race>,
}

impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (time, distance) = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .dropping(1)
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_vec()
            })
            .collect_tuple()
            .unwrap();

        let races = time
            .into_iter()
            .zip(distance)
            .map(|(time, distance)| Race { time, distance })
            .collect_vec();

        Ok(Puzzle { races })
    }
}

fn part1(data: &str) -> Result<u64> {
    let races = data.parse::<Puzzle>()?.races;

    let product = races.iter().map(ways_to_beat).product::<u64>();

    Ok(product)
}

#[deprecated]
#[allow(dead_code)]
fn ways_to_beat_slow(race: &Race) -> u64 {
    (0..=race.time)
        .map(|hold| {
            let speed = hold;
            speed * (race.time - hold) // dist
        })
        .filter(|dist| *dist > race.distance)
        .count() as u64
}

fn ways_to_beat(race: &Race) -> u64 {
    // x: time
    // y: distance
    // T = race.time
    // D = race.distance + 1
    // we're looking for roots of: - x^2 + T * x - D = 0
    // i.e. quadratic roots of a=-1, b=T, c=-D
    // which are: x = (-b +- sqrt(b^2 - 4ac)) / 2a
    let a = -1.0;
    let b = try_f64_from_u64(race.time).unwrap();
    let c = -try_f64_from_u64(race.distance + 1).unwrap();
    let roots = quadratic_roots(a, b, c);
    let (x1, x2) = roots;
    let x1 = try_u64_from_f64(x1.ceil()).unwrap();
    let x2 = try_u64_from_f64(x2.floor()).unwrap();
    x2 - x1 + 1
}

fn quadratic_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b.powi(2) - 4.0 * a * c;
    let root = discriminant.sqrt();
    let x1 = (-b + root) / (2.0 * a);
    let x2 = (-b - root) / (2.0 * a);
    (x1, x2)
}

fn part2(data: &str) -> Result<u64> {
    let races = data.parse::<Puzzle>()?.races;
    let time = races.iter().map(|r| r.time).join("").parse::<u64>()?;
    let distance = races.iter().map(|r| r.distance).join("").parse::<u64>()?;
    let race = Race { time, distance };

    let ways_to_beat = ways_to_beat(&race);
    Ok(ways_to_beat)
}
