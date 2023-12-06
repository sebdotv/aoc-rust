use anyhow::Result;
use itertools::Itertools;
use std::str::FromStr;

use crate::challenge::Day;

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

    let product = races.iter().map(ways_to_beat).product::<usize>();

    Ok(u64::try_from(product)?)
}

fn ways_to_beat(race: &Race) -> usize {
    (0..=race.time)
        .map(|hold| {
            let speed = hold;
            speed * (race.time - hold) // dist
        })
        .filter(|dist| *dist > race.distance)
        .count()
}

fn part2(data: &str) -> Result<u64> {
    let races = data.parse::<Puzzle>()?.races;
    let time = races.iter().map(|r| r.time).join("").parse::<u64>()?;
    let distance = races.iter().map(|r| r.distance).join("").parse::<u64>()?;
    let race = Race { time, distance };

    let ways_to_beat = ways_to_beat(&race);
    Ok(u64::try_from(ways_to_beat)?)
}
