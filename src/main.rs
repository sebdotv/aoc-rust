use std::fmt::Debug;
use std::time::{Duration, Instant};

use anyhow::Result;
use chrono::Datelike;
use clap::Parser;
use colored::Colorize;
use itertools::Itertools;
use strum::IntoEnumIterator;

use aoc_rust::all_challenge_days;
use aoc_rust::challenge::{Day, DayWrapper, Part};

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    year: Option<i32>,
    #[arg(long)]
    day: Option<u32>,

    /// Whether only the latest days should be checked (default: all available days)
    #[arg(long)]
    latest: bool,

    #[arg(long)]
    part: Option<u8>,

    #[arg(long, value_enum)]
    only: Option<Only>,
}

#[derive(Debug, Copy, Clone, clap::ValueEnum, Eq, PartialEq)]
#[clap(rename_all = "lowercase")]
enum Only {
    Example,
    Input,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut days = all_challenge_days();

    days = days
        .into_iter()
        .filter(|day| {
            let date = day.source_file_location().unwrap().date().unwrap();
            if let Some(year) = args.year {
                if date.year() != year {
                    return false;
                }
            }
            if let Some(day) = args.day {
                if date.day() != day {
                    return false;
                }
            }
            true
        })
        .collect_vec();

    let challenge_days = if args.latest {
        days.into_iter().rev().take(1).collect_vec()
    } else {
        days
    };
    let part = args.part.map(Part::try_from).transpose()?;
    for ref day in challenge_days {
        match day {
            DayWrapper::I32(day) => check_day(day, part, args.only)?,
            DayWrapper::U32(day) => check_day(day, part, args.only)?,
            DayWrapper::U64(day) => check_day(day, part, args.only)?,
            DayWrapper::Usize(day) => check_day(day, part, args.only)?,
            DayWrapper::String(day) => check_day(day, part, args.only)?,
        }
    }
    Ok(())
}

fn check_day<T>(day: &Day<T>, part_filter: Option<Part>, only: Option<Only>) -> Result<()>
where
    T: Eq + Debug,
{
    let example_data = if day.distinct_examples {
        None
    } else {
        Some(day.read_data_file("example")?)
    };
    let input_data = day.read_data_file("input")?;
    for part in Part::iter() {
        if part_filter.is_some() && part != part_filter.unwrap() {
            continue;
        }
        let part_example_data = if example_data.is_none() {
            let file_name = format!("example{}", part as u8);
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
            only,
        )?;
    }
    Ok(())
}

fn check_part<T>(
    day: &Day<T>,
    part: Part,
    example_data: &str,
    input_data: &str,
    only: Option<Only>,
) -> Result<bool>
where
    T: Eq + Debug,
{
    let mut ok = true;
    if let Some((example_solution, input_solution)) = day.solutions(part) {
        if only != Some(Only::Input) {
            ok &= check_value(
                day,
                part,
                "example",
                Some(example_solution),
                solve_and_measure(day, part, example_data)?,
            );
            if !ok {
                return Ok(ok);
            }
        }
        if only != Some(Only::Example) {
            ok &= check_value(
                day,
                part,
                "input",
                input_solution.as_ref(),
                solve_and_measure(day, part, input_data)?,
            );
        }
    }
    Ok(ok)
}

fn solve_and_measure<T>(day: &Day<T>, part: Part, data: &str) -> Result<(T, Duration)> {
    let solver = day.get_solver(part);
    let start = Instant::now();
    let value = solver(data)?;
    let duration = start.elapsed();
    Ok((value, duration))
}

fn check_value<T>(
    day: &Day<T>,
    part: Part,
    label: &str,
    expected: Option<&T>,
    actual_result: (T, Duration),
) -> bool
where
    T: Eq + Debug,
{
    let (actual, duration) = actual_result;
    let duration_str = || {
        let text = format!("{:.1} µs", duration.as_secs_f64() * 1e6);
        (if duration.as_millis() >= 200 {
            text.bold().truecolor(255, 83, 0) // orange
        } else {
            text.yellow()
        })
        .to_string()
    };
    let (status, details, ok) = if let Some(expected) = expected {
        if actual == *expected {
            ("OK".green(), duration_str(), true)
        } else {
            (
                "FAIL".red().bold(),
                format!(
                    "expected {}, got {}",
                    format!("{expected:?}").green(),
                    format!("{actual:?}").red(),
                ),
                false,
            )
        }
    } else {
        (
            "NEW".cyan(),
            format!(
                "{} {} (not checked)",
                duration_str(),
                format!("{actual:?}").cyan().bold(),
            ),
            true,
        )
    };
    println!(
        "{} {} {} [{}]",
        status,
        day.label(part).unwrap(),
        label,
        details
    );
    ok
}
