use std::fmt::Debug;

use criterion::{criterion_group, criterion_main, Criterion};
use strum::IntoEnumIterator;

use aoc_rust::challenge::{ChallengeDay, ChallengeDayType, Part};
use aoc_rust::*;

fn bench_challenge_days(c: &mut Criterion) {
    for day in all_challenge_days() {
        match day {
            ChallengeDayType::I32(day) => bench_day(c, &day),
            ChallengeDayType::U32(day) => bench_day(c, &day),
            ChallengeDayType::String(day) => bench_day(c, &day),
        }
    }
}

fn bench_day<T>(c: &mut Criterion, day: &ChallengeDay<T>)
where
    T: Eq + Debug,
{
    let input_data = day.read_data_file("input").unwrap();
    for ref part in Part::iter() {
        if let Some((_, Some(expected_value))) = day.solutions(part) {
            let solver = day.get_solver(part);
            c.bench_function(day.label(part).as_str(), |b| {
                b.iter(|| assert_eq!(expected_value, &solver(&input_data).unwrap()))
            });
        }
    }
}

criterion_group!(benches, bench_challenge_days);
criterion_main!(benches);
