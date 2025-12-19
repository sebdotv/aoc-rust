use anyhow::{Result, bail};

use crate::challenge::Day;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (3, Some(980)),
        part2_solutions: Some((6, Some(5961))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let mut zeroes: usize = 0;
    let mut dial: i32 = 50;
    for line in data.lines() {
        let (dir, n) = line.split_at(1);
        let n: i32 = n.parse()?;
        dial = match dir {
            "L" => dial - n,
            "R" => dial + n,
            _ => bail!("Invalid direction"),
        };
        if dial % 100 == 0 {
            zeroes += 1;
        }
    }
    Ok(zeroes)
}

fn part2(data: &str) -> Result<usize> {
    let mut zeroes: usize = 0;
    let mut dial: i32 = 50;
    for line in data.lines() {
        let (dir, n) = line.split_at(1);
        let n: i32 = n.parse()?;
        let dial_incr = match dir {
            "L" => -1,
            "R" => 1,
            _ => bail!("Invalid direction"),
        };
        for _ in 0..n {
            dial += dial_incr;
            if dial % 100 == 0 {
                zeroes += 1;
            }
        }
    }
    Ok(zeroes)
}
