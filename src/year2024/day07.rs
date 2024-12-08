use crate::challenge::Day;
use anyhow::Result;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (3749, Some(538191549061)),
        part2_solutions: Some((11387, Some(34612812972206))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    run_part(data, false)
}

fn part2(data: &str) -> Result<usize> {
    run_part(data, true)
}

type Equation = (usize, Vec<usize>);

fn run_part(data: &str, with_concat: bool) -> Result<usize> {
    Ok(data
        .lines()
        .map(parse_equation)
        .filter_map(|eq| solve_equation(&eq, with_concat).then_some(eq.0))
        .sum())
}

fn parse_equation(line: &str) -> Equation {
    let (left, right) = line.split_once(": ").unwrap();
    let left = left.parse().unwrap();
    let right = right.split(' ').map(|s| s.parse().unwrap()).collect();
    (left, right)
}

fn solve_equation(eq: &Equation, with_concat: bool) -> bool {
    fn it(left: usize, remaining: &[usize], value: usize, with_concat: bool) -> bool {
        if remaining.is_empty() {
            return value == left;
        }
        if value > left {
            return false;
        }
        let head = remaining[0];
        let tail = &remaining[1..];
        it(left, tail, value + head, with_concat)
            || it(left, tail, value * head, with_concat)
            || with_concat
                && it(
                    left,
                    tail,
                    value * 10usize.pow(head.checked_ilog10().unwrap() + 1) + head,
                    with_concat,
                )
    }

    let (left, right) = eq;
    it(*left, &right[1..], right[0], with_concat)
}
