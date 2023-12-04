use crate::challenge::ChallengeDayType;

mod day01;
mod day04;

pub fn challenge_days() -> Vec<ChallengeDayType> {
    vec![day01::day().into(), day04::day().into()]
}
