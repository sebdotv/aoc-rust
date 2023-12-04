use std::fmt::Debug;
use std::time::{Duration, Instant};

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use itertools::Itertools;
use strum::IntoEnumIterator;

use aoc_rust::all_challenge_days;
use aoc_rust::challenge::{ChallengeDay, ChallengeDayType, Part};

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    /// Whether only the latest days should be checked (default: all available days)
    #[arg(long)]
    only_latest: bool,

    /// Whether only the examples should be checked
    #[arg(long)]
    only_example: bool,
}
fn main() -> Result<()> {
    let args = Args::parse();

    let challenge_days = if !args.only_latest {
        all_challenge_days()
    } else {
        all_challenge_days().into_iter().rev().take(1).collect_vec()
    };
    for ref day in challenge_days {
        match day {
            ChallengeDayType::I32(day) => check_day(day, args.only_example)?,
            ChallengeDayType::U32(day) => check_day(day, args.only_example)?,
            ChallengeDayType::String(day) => check_day(day, args.only_example)?,
        }
    }
    Ok(())
}

fn check_day<T>(day: &ChallengeDay<T>, only_example: bool) -> Result<()>
where
    T: Eq + Debug,
{
    let example_data = if !day.distinct_examples {
        Some(day.read_data_file("example")?)
    } else {
        None
    };
    let input_data = day.read_data_file("input")?;
    for ref part in Part::iter() {
        let part_example_data = if example_data.is_none() {
            let file_name = format!("example{}", *part as u8);
            Some(day.read_data_file(file_name.as_str())?)
        } else {
            None
        };
        check_part(
            day,
            part,
            part_example_data
                .as_ref()
                .or(example_data.as_ref())
                .unwrap()
                .as_str(),
            &input_data,
            only_example,
        )?;
    }
    Ok(())
}

fn check_part<T>(
    day: &ChallengeDay<T>,
    part: &Part,
    example_data: &str,
    input_data: &str,
    only_example: bool,
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
        if !only_example {
            check_value(
                day,
                part,
                "input",
                input_solution.as_ref(),
                solve_and_measure(day, part, input_data)?,
            );
        }
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
    let duration_str = || {
        format!("{:.1} µs", duration.as_secs_f64() * 1e6)
            .yellow()
            .to_string()
    };
    let (status, details) = if let Some(expected) = expected {
        if actual == *expected {
            ("OK".green(), duration_str())
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
            format!(
                "{} {} (not checked)",
                duration_str(),
                format!("{:?}", actual).cyan().bold(),
            ),
        )
    };
    println!("{} {} {} [{}]", status, day.label(part), label, details);
}
