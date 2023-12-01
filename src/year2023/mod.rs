use crate::challenge::ChallengeDayType;

mod day01;

pub fn challenge_days() -> Vec<ChallengeDayType> {
    vec![day01::day().into()]
}
