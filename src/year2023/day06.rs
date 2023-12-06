use anyhow::Result;
use itertools::Itertools;
use std::str::FromStr;

use crate::challenge::Day;

pub fn day() -> Day<u32> {
    Day {
        part1_solutions: (288, None),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

// race
// time (ms)
// distance (mm)
// hold button: charge boat
// release button: allow boat to move
// starting speed: 0 mm/ms
// hold button: for each 1 ms, speed += 1 mm/ms

struct Race {
    time: u32,
    distance: u32,
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
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect_vec()
            })
            .collect_tuple()
            .unwrap();

        println!("{:?}", time);
        println!("{:?}", distance);

        let races = time
            .into_iter()
            .zip(distance)
            .map(|(time, distance)| Race { time, distance })
            .collect_vec();

        Ok(Puzzle { races })
    }
}

fn part1(data: &str) -> Result<u32> {
    let races = data.parse::<Puzzle>()?.races;

    let product = races.iter().map(ways_to_beat).product::<usize>();

    Ok(u32::try_from(product)?)
}

fn ways_to_beat(race: &Race) -> usize {
    (0..=race.time)
        .map(|hold| {
            let speed = hold;
            let dist = speed * (race.time - hold);
            println!("hold: {}, speed: {}, dist: {}", hold, speed, dist);
            dist
        })
        .filter(|dist| *dist > race.distance)
        .count()
}

fn part2(_data: &str) -> Result<u32> {
    todo!()
}
