use std::fmt::Debug;
use std::time::{Duration, Instant};

use anyhow::Result;
use colored::Colorize;
use strum::IntoEnumIterator;

use aoc_rust::all_challenge_days;
use aoc_rust::challenge::{ChallengeDay, ChallengeDayType, Part};

fn main() -> Result<()> {
    for ref day in all_challenge_days().iter().rev().take(1) {
        match day {
            ChallengeDayType::I32(day) => check_day(day)?,
            ChallengeDayType::U32(day) => check_day(day)?,
            ChallengeDayType::String(day) => check_day(day)?,
        }
    }
    Ok(())
}

fn check_day<T>(day: &ChallengeDay<T>) -> Result<()>
    where
        T: Eq + Debug,
{
    let example_data = day.read_data_file("example")?;
    let input_data = day.read_data_file("input")?;
    for ref part in Part::iter() {
        check_part(day, part, &example_data, &input_data)?;
    }
    Ok(())
}

fn check_part<T>(
    day: &ChallengeDay<T>,
    part: &Part,
    example_data: &str,
    input_data: &str,
) -> Result<()>
    where
        T: Eq + Debug,
{
    if let Some((example_solution, input_solution)) = day.solutions(part) {
        check_value(
            day,
            part,
            "example",
            Some(example_solution),
            solve_and_measure(day, part, example_data)?,
        );
        check_value(
            day,
            part,
            "input",
            input_solution.as_ref(),
            solve_and_measure(day, part, input_data)?,
        );
    }
    Ok(())
}

fn solve_and_measure<T>(day: &ChallengeDay<T>, part: &Part, data: &str) -> Result<(T, Duration)> {
    let solver = day.get_solver(part);
    let start = Instant::now();
    let value = solver(data)?;
    let duration = start.elapsed();
    Ok((value, duration))
}

fn check_value<T>(
    day: &ChallengeDay<T>,
    part: &Part,
    label: &str,
    expected: Option<&T>,
    actual_result: (T, Duration),
) where
    T: Eq + Debug,
{
    let (actual, duration) = actual_result;
    let duration_str = || format!("{:.1} µs", duration.as_secs_f64() * 1e6)
        .yellow()
        .to_string();
    let (status, details) = if let Some(expected) = expected {
        if actual == *expected {
            (
                "OK".green(),
                duration_str(),
            )
        } else {
            (
                "FAIL".red().bold(),
                format!(
                    "expected {}, got {}",
                    format!("{:?}", expected).green(),
                    format!("{:?}", actual).red(),
                ),
            )
        }
    } else {
        (
            "NEW".cyan(),
            format!("{} {} (not checked)",
                    duration_str(),
                    format!("{:?}", actual).cyan().bold(),
            )
        )
    };
    println!("{} {} {} [{}]", status, day.label(part), label, details);
}
