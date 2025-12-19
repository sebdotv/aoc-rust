use crate::challenge::Day;
use crate::utils::f64_conversions::try_usize_from_f64;
use anyhow::{Result, anyhow, ensure};
use itertools::Itertools;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, value};
use nom::multi::{many1, separated_list1};
use nom::sequence::delimited;
use std::str::FromStr;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (7, Some(558)),
        part2_solutions: Some((33, Some(20317))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    use pathfinding::prelude::bfs;

    let machine_defs = data
        .lines()
        .map(str::parse::<MachineDef>)
        .collect::<Result<Vec<_>>>()?;

    let mut sum = 0;

    for md in &machine_defs {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        struct Pos {
            lights: Vec<bool>,
        }
        impl Pos {
            fn successors(&self, md: &MachineDef) -> Vec<Self> {
                let mut succs = Vec::new();
                for btn in &md.buttons {
                    succs.push(self.apply_button(btn));
                }
                succs
            }
            fn apply_button(&self, btn: &Vec<usize>) -> Self {
                let mut new_lights = self.lights.clone();
                for &idx in btn {
                    new_lights[idx] = !new_lights[idx];
                }
                Self { lights: new_lights }
            }
        }
        let start = Pos {
            lights: vec![false; md.light_diag.len()],
        };
        let goal = Pos {
            lights: md.light_diag.clone(),
        };
        let result = bfs(&start, |p| p.successors(md), |p| *p == goal).unwrap();
        sum += result.len() - 1;
    }

    Ok(sum)
}

fn part2(data: &str) -> Result<usize> {
    use good_lp::Expression;
    use good_lp::{IntoAffineExpression, ProblemVariables, SolverModel, highs, variable};

    let machine_defs = data
        .lines()
        .map(str::parse::<MachineDef>)
        .collect::<Result<Vec<_>>>()?;

    let mut sum = 0;

    for md in &machine_defs {
        let mut vars = ProblemVariables::new();

        // variables for number of times each button is pressed
        let var_btns = (0..md.buttons.len())
            .map(|_| vars.add(variable().integer().min(0)))
            .collect_vec();

        // constraints for each joltage counter
        let constraints = md
            .joltage_reqs
            .iter()
            .enumerate()
            .map(|(idx, req)| {
                let mut joltage_expr = 0.into_expression();
                for (btn, var_btn) in md.buttons.iter().zip(var_btns.iter()) {
                    for x in btn {
                        if *x == idx {
                            joltage_expr += var_btn;
                        }
                    }
                }

                joltage_expr.eq(i32::try_from(*req).unwrap())
            })
            .collect_vec();

        let total_presses: Expression = var_btns.iter().sum();
        let mut problem = vars.minimise(&total_presses).using(highs);
        for c in constraints {
            problem.add_constraint(c);
        }
        let solution = problem.solve()?;
        let sum_solution = total_presses.eval_with(&solution);

        sum += try_usize_from_f64(sum_solution.round()).unwrap();
    }

    Ok(sum)
}

#[derive(Debug)]
struct MachineDef {
    light_diag: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_reqs: Vec<usize>,
}
impl MachineDef {
    fn new(
        light_diag: Vec<bool>,
        buttons: Vec<Vec<usize>>,
        joltage_reqs: Vec<usize>,
    ) -> Result<Self> {
        ensure!(!light_diag.is_empty());
        for btn in &buttons {
            ensure!(btn.iter().all(|&b| b < light_diag.len()),);
        }
        ensure!(joltage_reqs.len() == light_diag.len());
        Ok(Self {
            light_diag,
            buttons,
            joltage_reqs,
        })
    }
}
impl FromStr for MachineDef {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, md) =
            machine_def(s).map_err(|e| anyhow!("Failed to parse MachineDef: {:?}", e))?;
        ensure!(input.is_empty());
        Ok(md)
    }
}

// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
fn machine_def(input: &str) -> IResult<&str, MachineDef> {
    let (input, light_diag) = light_diag(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, buttons) = buttons(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, joltage_reqs) = joltage_reqs(input)?;

    let md = MachineDef::new(light_diag, buttons, joltage_reqs).map_err(|_e| {
        nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Verify))
    })?;
    Ok((input, md))
}

// e.g. [.##.]
fn light_diag(input: &str) -> IResult<&str, Vec<bool>> {
    delimited(
        tag("["),
        many1(alt((value(false, tag(".")), value(true, tag("#"))))),
        tag("]"),
    )
    .parse(input)
}

// e.g. (3) (1,3) (2) (2,3) (0,2) (0,1)
fn buttons(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(
        tag(" "),
        delimited(
            tag("("),
            separated_list1(tag(","), map_res(digit1, usize::from_str)),
            tag(")"),
        ),
    )
    .parse(input)
}

// e.g. {3,5,4,7}
fn joltage_reqs(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        tag("{"),
        separated_list1(tag(","), map_res(digit1, usize::from_str)),
        tag("}"),
    )
    .parse(input)
}
