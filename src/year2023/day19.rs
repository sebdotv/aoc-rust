use std::fmt::Display;
use std::str::FromStr;

use anyhow::Result;
use indexmap::IndexMap;
use itertools::Itertools;
use strum_macros::{EnumIter, EnumString};

use crate::challenge::Day;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (19114, Some(406849)),
        part2_solutions: Some((167409079868000, Some(138625360533574))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let puzzle = data.parse::<Puzzle>()?;

    let mut accepted = 0;

    for part in &puzzle.parts {
        let mut workflow_name = "in";

        loop {
            let workflow = puzzle.workflows.get(workflow_name).unwrap();
            let action = workflow.apply(part);
            match action {
                Action::Accept => {
                    accepted += part.score();
                    break;
                }
                Action::Reject => break,
                Action::GotoWorkflow(next_wf) => workflow_name = next_wf,
            }
        }
    }

    Ok(accepted)
}

fn part2(data: &str) -> Result<usize> {
    let puzzle = data.parse::<Puzzle>()?;
    let solver = Solver::new(puzzle);
    let paths = solver.paths("in");
    Ok(paths.iter().map(Path::combinations).sum())
}

struct Solver {
    puzzle: Puzzle,
}
impl Solver {
    fn new(puzzle: Puzzle) -> Self {
        Self { puzzle }
    }
    fn paths(&self, workflow_name: &str) -> Vec<Path> {
        let workflow = self.puzzle.workflows.get(workflow_name).unwrap();

        let mut paths = vec![];
        let mut negative_constraints = vec![];

        for rule in &workflow.rules {
            paths.extend(self.paths_for_action(&rule.action).iter().map(|path| {
                let mut path = path.clone();
                path.constraints.push(rule.constraint);
                path.negative_constraints.extend(&negative_constraints);
                path
            }));
            negative_constraints.push(rule.constraint);
        }

        paths.extend(
            self.paths_for_action(&workflow.default_action)
                .iter()
                .map(|path| {
                    let mut path = path.clone();
                    path.negative_constraints.extend(&negative_constraints);
                    path
                }),
        );

        paths
    }
    fn paths_for_action(&self, action: &Action) -> Vec<Path> {
        match action {
            Action::Accept => vec![Path {
                constraints: vec![],
                negative_constraints: vec![],
            }],
            Action::Reject => vec![],
            Action::GotoWorkflow(other) => self.paths(other),
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
    constraints: Vec<Constraint>,
    negative_constraints: Vec<Constraint>,
}
impl Path {
    const MIN: usize = 1;
    const MAX: usize = 4000;
    fn combinations(&self) -> usize {
        let mut possible_values = [[true; Self::MAX]; 4];
        for constraint in &self.constraints {
            let range = match constraint.operator {
                Operator::Lt => constraint.value..=Self::MAX,
                Operator::Gt => Self::MIN..=constraint.value,
            };
            let cat_idx = constraint.category as usize;
            for i in range {
                possible_values[cat_idx][i - 1] = false;
            }
        }
        for constraint in &self.negative_constraints {
            let range = match constraint.operator {
                Operator::Lt => Self::MIN..constraint.value,
                #[allow(clippy::range_plus_one)]
                Operator::Gt => (constraint.value + 1)..Self::MAX + 1,
            };
            let cat_idx = constraint.category as usize;
            for i in range {
                possible_values[cat_idx][i - 1] = false;
            }
        }
        let mut product = 1;
        for possible_values_for_cat in possible_values {
            let count = possible_values_for_cat.iter().filter(|&&b| b).count();
            product *= count;
        }
        product
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let constraints = self.constraints.iter().map(ToString::to_string).join(", ");
        let negative_constraints = self
            .negative_constraints
            .iter()
            .map(ToString::to_string)
            .join(", ");
        write!(
            f,
            "constraints: [{}], negative_constraints: [{}]",
            constraints, negative_constraints
        )
    }
}

#[derive(Debug)]
struct Puzzle {
    workflows: IndexMap<String, Workflow>,
    parts: Vec<Part>,
}
impl FromStr for Puzzle {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let lines = s.lines().collect_vec();
        let (workflows, parts) = lines.split(|s| s.is_empty()).collect_tuple().unwrap();
        let workflows = workflows
            .iter()
            .map(|s| s.parse::<Workflow>().unwrap())
            .map(|w| (w.name.clone(), w))
            .collect();
        let parts = parts
            .iter()
            .map(|s| s.parse::<Part>().unwrap())
            .collect_vec();
        Ok(Self { workflows, parts })
    }
}

#[derive(EnumString, Debug, Eq, PartialEq, strum_macros::Display, Copy, Clone, EnumIter, Hash)]
#[strum(serialize_all = "lowercase")]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Part {
    ratings: IndexMap<Category, usize>,
}
impl Part {
    fn score(&self) -> usize {
        self.ratings.values().sum()
    }
}
impl FromStr for Part {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let s = s.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
        let ratings = s
            .split(',')
            .map(|s| {
                let (cat, rating) = s.split_once('=').unwrap();
                let cat = cat.parse::<Category>().unwrap();
                let rating = rating.parse::<usize>().unwrap();
                (cat, rating)
            })
            .collect();
        Ok(Self { ratings })
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default_action: Action,
}
impl Workflow {
    fn apply(&self, part: &Part) -> &Action {
        for rule in &self.rules {
            let Constraint {
                category,
                operator,
                value,
            } = &rule.constraint;
            let rating = part.ratings.get(category).unwrap();
            let matches = match operator {
                Operator::Lt => rating < value,
                Operator::Gt => rating > value,
            };
            if matches {
                return &rule.action;
            }
        }
        &self.default_action
    }
}
impl FromStr for Workflow {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let s = s.strip_suffix('}').unwrap();
        let (name, right) = s.split_once('{').unwrap();
        let name = name.to_owned();
        let items = right.split(',').collect_vec();
        let (default_action, rules) = items.split_last().unwrap();
        let default_action = default_action.parse::<Action>().unwrap();
        let rules = rules
            .iter()
            .map(|s| s.parse::<Rule>().unwrap())
            .collect_vec();
        Ok(Self {
            name,
            rules,
            default_action,
        })
    }
}

#[derive(Debug, Clone)]
enum Action {
    Accept,
    Reject,
    GotoWorkflow(String),
}
impl FromStr for Action {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            s => Ok(Self::GotoWorkflow(s.to_owned())),
        }
    }
}

#[derive(Debug)]
struct Rule {
    constraint: Constraint,
    action: Action,
}
impl FromStr for Rule {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let (left, action) = s.split_once(':').unwrap();

        let (category, rest) = left.split_at(1);
        let category = category.parse::<Category>().unwrap();
        let (operator, value) = rest.split_at(1);
        let operator = operator.parse::<Operator>().unwrap();
        let value = value.parse::<usize>().unwrap();

        let constraint = Constraint {
            category,
            operator,
            value,
        };
        let action = action.parse::<Action>().unwrap();

        Ok(Self { constraint, action })
    }
}
#[derive(Debug, Copy, Clone)]
struct Constraint {
    category: Category,
    operator: Operator,
    value: usize,
}
impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self.operator {
            Operator::Lt => "<",
            Operator::Gt => ">",
        };
        write!(f, "{}{}{}", self.category, op, self.value)
    }
}

#[derive(EnumString, Debug, Eq, PartialEq, strum_macros::Display, Copy, Clone, EnumIter)]
enum Operator {
    #[strum(serialize = "<")]
    Lt,
    #[strum(serialize = ">")]
    Gt,
}
