use crate::challenge::Day;
use anyhow::Result;
use itertools::Itertools;
use pathfinding::prelude::count_paths;
use std::collections::HashMap;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (5, Some(796)),
        part2_solutions: Some((2, Some(294053029111296))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: true,
    }
}

fn part1(data: &str) -> Result<usize> {
    let flow = parse(data);
    let successors = |pos: &String| flow.get(pos).unwrap().iter().cloned();
    let start = "you".to_owned();
    let count = count_paths(start, successors, |pos| *pos == "out");
    Ok(count)
}

fn part2(data: &str) -> Result<usize> {
    #[derive(Eq, PartialEq, Hash, Clone)]
    struct Pos {
        node: String,
        visited_dac: bool,
        visited_fft: bool,
    }
    impl Pos {
        fn start() -> Self {
            Self {
                node: "svr".to_owned(),
                visited_dac: false,
                visited_fft: false,
            }
        }
        fn successors(&self, flow: &HashMap<String, Vec<String>>) -> Vec<Self> {
            if let Some(outputs) = flow.get(&self.node) {
                outputs
                    .iter()
                    .map(|next_node| Pos {
                        node: next_node.clone(),
                        visited_dac: self.visited_dac || next_node == "dac",
                        visited_fft: self.visited_fft || next_node == "fft",
                    })
                    .collect_vec()
            } else {
                vec![]
            }
        }
        fn success(&self) -> bool {
            self.node == "out" && self.visited_dac && self.visited_fft
        }
    }

    let flow = parse(data);
    let count = count_paths(Pos::start(), |pos| pos.successors(&flow), Pos::success);
    Ok(count)
}

fn parse(data: &str) -> HashMap<String, Vec<String>> {
    data.lines()
        .map(|line| {
            let (device, rest) = line.split_once(':').unwrap();
            let outputs = rest.split_whitespace().map(ToOwned::to_owned).collect_vec();
            (device.to_owned(), outputs)
        })
        .collect()
}
