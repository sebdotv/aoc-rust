use anyhow::Result;

use crate::challenge::ChallengeDay;

pub fn day() -> ChallengeDay<i32> {
    ChallengeDay {
        part1_solutions: (todo!(), None),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
    }
}

fn part1(data: &str) -> Result<i32> {
    todo!()
}

fn part2(_data: &str) -> Result<i32> {
    todo!()
}
