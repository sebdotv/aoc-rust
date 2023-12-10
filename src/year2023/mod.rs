use crate::challenge::DayWrapper;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

#[must_use]
pub fn challenge_days() -> Vec<DayWrapper> {
    vec![
        day01::day().into(),
        day02::day().into(),
        day03::day().into(),
        day04::day().into(),
        day05::day().into(),
        day06::day().into(),
        day07::day().into(),
        day08::day().into(),
        day09::day().into(),
    ]
}
