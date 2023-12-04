use crate::challenge::DayWrapper;

mod day01;
mod day02;
mod day03;
mod day04;

#[must_use]
pub fn challenge_days() -> Vec<DayWrapper> {
    vec![
        day01::day().into(),
        day02::day().into(),
        day03::day().into(),
        day04::day().into(),
    ]
}
