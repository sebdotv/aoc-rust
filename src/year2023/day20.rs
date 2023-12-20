use anyhow::{bail, Result};
use indexmap::map::Entry;
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::Display;
use std::str::FromStr;

use crate::challenge::Day;

pub fn day() -> Day<i32> {
    Day {
        part1_solutions: (32000000, None),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<i32> {
    let puzzle: Puzzle = data.parse()?;
    let modules = create_modules(puzzle);
    run_simulation(modules);

    Ok(0)
}

fn run_simulation(mut modules: IndexMap<String, Box<dyn Module>>) -> Vec<Pulse> {
    let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();
    pulse_queue.push_back(Pulse {
        destination: "broadcaster".to_owned(),
        high: false,
        source: "button".to_owned(),
    });

    let mut processed_pulses: Vec<Pulse> = vec![];

    loop {
        let pulse = pulse_queue.pop_front();
        if pulse.is_none() {
            break;
        }
        let pulse = pulse.unwrap();
        processed_pulses.push(pulse.clone());

        // println!("processing pulse {:?}", pulse);

        let module = modules.get_mut(&pulse.destination).unwrap();
        let new_pulses = module.process(pulse);

        // println!("new pulses: {:?}", new_pulses);

        new_pulses
            .into_iter()
            .for_each(|pulse| pulse_queue.push_back(pulse));
    }

    processed_pulses
}

fn create_modules(puzzle: Puzzle) -> IndexMap<String, Box<dyn Module>> {
    let mut modules: IndexMap<String, Box<dyn Module>> = puzzle
        .modules
        .into_iter()
        .map(|cfg| {
            let name = cfg.name.clone();
            println!("creating module {:?}", name);
            let enriched_cfg = EnrichedModuleConfig {
                module_config: cfg,
                inputs: puzzle.inputs.get(&name).unwrap_or(&vec![]).clone(),
            };
            let module: Box<dyn Module> = enriched_cfg.into();
            (name, module)
        })
        .collect();
    modules
}

fn part2(_data: &str) -> Result<i32> {
    todo!()
}

#[derive(Debug)]
struct Puzzle {
    modules: Vec<ModuleConfig>,
    inputs: IndexMap<String, Vec<String>>,
}
impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let modules: Vec<ModuleConfig> = s.lines().map(str::parse).collect::<Result<Vec<_>>>()?;

        let mut inputs: IndexMap<String, Vec<String>> = IndexMap::new();

        for module in &modules {
            for destination in &module.destinations {
                inputs
                    .entry(destination.to_owned())
                    .or_default()
                    .push(module.name.clone());
            }
        }

        Ok(Self { modules, inputs })
    }
}
#[derive(Debug)]
struct ModuleConfig {
    r#type: ModuleType,
    name: String,
    destinations: Vec<String>,
}
#[derive(Debug)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}
impl FromStr for ModuleConfig {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (left, destinations) = s.split_once(" -> ").unwrap();
        let (r#type, name) = if left == "broadcaster" {
            (ModuleType::Broadcast, "broadcaster")
        } else if let Some(name) = left.strip_prefix('%') {
            (ModuleType::FlipFlop, name)
        } else if let Some(name) = left.strip_prefix('&') {
            (ModuleType::Conjunction, name)
        } else {
            bail!("invalid module type: {}", left);
        };
        let name = name.to_owned();
        let destinations = destinations
            .split(", ")
            .map(ToOwned::to_owned)
            .collect_vec();
        Ok(Self {
            r#type,
            name,
            destinations,
        })
    }
}

struct EnrichedModuleConfig {
    module_config: ModuleConfig,
    inputs: Vec<String>,
}

impl From<EnrichedModuleConfig> for Box<dyn Module> {
    fn from(enriched_cfg: EnrichedModuleConfig) -> Box<dyn Module> {
        let cfg = enriched_cfg.module_config;
        let base_module = BaseModule {
            destinations: cfg.destinations,
        };
        match cfg.r#type {
            ModuleType::Broadcast => Box::new(BroadcastModule { base_module }),
            ModuleType::FlipFlop => Box::new(FlipFlopModule {
                base_module,
                on: false,
            }),
            ModuleType::Conjunction => Box::new(ConjunctionModule {
                base_module,
                last_received_from_was_high: enriched_cfg
                    .inputs
                    .into_iter()
                    .map(|name| (name, false))
                    .collect(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
struct Pulse {
    destination: String,
    high: bool,
    source: String,
}
impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -{}-> {}",
            self.source,
            if self.high { "high" } else { "low" },
            self.destination
        )
    }
}

trait Module {
    fn process(&mut self, pulse: Pulse) -> Vec<Pulse>;
}

struct BaseModule {
    destinations: Vec<String>,
}
impl BaseModule {
    fn new_pulses_from(&self, high: bool, source: String) -> Vec<Pulse> {
        self.destinations
            .iter()
            .map(|name| Pulse {
                destination: name.to_owned(),
                high,
                source: source.clone(),
            })
            .collect_vec()
    }
}

struct BroadcastModule {
    base_module: BaseModule,
}
impl Module for BroadcastModule {
    fn process(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.base_module
            .new_pulses_from(pulse.high, pulse.destination)
    }
}

struct FlipFlopModule {
    base_module: BaseModule,
    on: bool,
}
impl Module for FlipFlopModule {
    fn process(&mut self, pulse: Pulse) -> Vec<Pulse> {
        if pulse.high {
            vec![]
        } else {
            self.on = !self.on;
            self.base_module.new_pulses_from(self.on, pulse.destination)
        }
    }
}

struct ConjunctionModule {
    base_module: BaseModule,
    last_received_from_was_high: IndexMap<String, bool>,
}
impl Module for ConjunctionModule {
    fn process(&mut self, pulse: Pulse) -> Vec<Pulse> {
        match self.last_received_from_was_high.entry(pulse.source.clone()) {
            Entry::Occupied(mut entry) => {
                entry.insert(pulse.high);
            }
            Entry::Vacant(_) => {
                panic!("received pulse from unknown source: {:?}", pulse);
            }
        }
        let high = self.last_received_from_was_high.values().all(|&high| high);
        self.base_module.new_pulses_from(high, pulse.destination)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::trim_lines;

    #[test]
    fn test_part1_example() {
        let data = day().read_data_file("example").unwrap();
        let puzzle: Puzzle = data.parse().unwrap();
        let modules = create_modules(puzzle);
        let actual = run_simulation(modules);
        let expected = r"
            button -low-> broadcaster
            broadcaster -low-> a
            broadcaster -low-> b
            broadcaster -low-> c
            a -high-> b
            b -high-> c
            c -high-> inv
            inv -low-> a
            a -low-> b
            b -low-> c
            c -low-> inv
            inv -high-> a        
        ";
        assert_eq!(
            actual.iter().map(ToString::to_string).join("\n"),
            trim_lines(expected)
        );
    }
}
