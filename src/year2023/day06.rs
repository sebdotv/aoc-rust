use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

use crate::challenge::Day;
use crate::utils::f64_conversions::{try_f64_from_usize, try_usize_from_f64};

pub fn day() -> Day<usize> {
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
    time: usize,
    distance: usize,
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
                    .map(|s| s.parse::<usize>().unwrap())
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

fn part1(data: &str) -> Result<usize> {
    let races = data.parse::<Puzzle>()?.races;

    let product = races.iter().map(ways_to_beat).product::<usize>();

    Ok(product)
}

#[deprecated]
#[allow(dead_code)]
fn ways_to_beat_slow(race: &Race) -> usize {
    (0..=race.time)
        .map(|hold| {
            let speed = hold;
            speed * (race.time - hold) // dist
        })
        .filter(|dist| *dist > race.distance)
        .count()
}

fn ways_to_beat(race: &Race) -> usize {
    // x: time
    // y: distance
    // T = race.time
    // D = race.distance + 1
    // we're looking for roots of: - x^2 + T * x - D = 0
    // i.e. quadratic roots of a=-1, b=T, c=-D
    // which are: x = (-b +- sqrt(b^2 - 4ac)) / 2a
    let a = -1.0;
    let b = try_f64_from_usize(race.time).unwrap();
    let c = -try_f64_from_usize(race.distance + 1).unwrap();
    let roots = quadratic_roots(a, b, c);
    let (x1, x2) = roots;
    let x1 = try_usize_from_f64(x1.ceil()).unwrap();
    let x2 = try_usize_from_f64(x2.floor()).unwrap();
    x2 - x1 + 1
}

fn quadratic_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b.powi(2) - 4.0 * a * c;
    let root = discriminant.sqrt();
    let x1 = (-b + root) / (2.0 * a);
    let x2 = (-b - root) / (2.0 * a);
    (x1, x2)
}

fn part2(data: &str) -> Result<usize> {
    let races = data.parse::<Puzzle>()?.races;
    let time = races.iter().map(|r| r.time).join("").parse::<usize>()?;
    let distance = races.iter().map(|r| r.distance).join("").parse::<usize>()?;
    let race = Race { time, distance };

    let ways_to_beat = ways_to_beat(&race);
    Ok(ways_to_beat)
}
