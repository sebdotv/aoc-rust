use anyhow::{bail, Result};
use indexmap::map::Entry;
use indexmap::IndexMap;
use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use crate::challenge::Day;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (32000000, Some(819397964)),
        part2_solutions: Some((1, None)), // actually there's no official example for part2
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: true,
    }
}

fn part1(data: &str) -> Result<usize> {
    let puzzle: Puzzle = data.parse()?;
    let mut sim = create_simulation(puzzle);
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        let pulses = push_button(&mut sim);
        for pulse in pulses {
            if pulse.high {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
        }
    }
    Ok(low_pulses * high_pulses)
}

fn part2(data: &str) -> Result<usize> {
    let puzzle: Puzzle = data.parse()?;
    let mut sim = create_simulation(puzzle);

    #[derive(Debug)]
    struct RxModule {
        low_received: bool,
    }
    impl Module for RxModule {
        fn process(&mut self, pulse: Pulse) -> Vec<Pulse> {
            if !pulse.high {
                self.low_received = true;
            }
            vec![]
        }
        fn done(&self) -> bool {
            self.low_received
        }
    }
    sim.modules.insert(
        "rx".to_owned(),
        Box::new(RxModule {
            low_received: false,
        }),
    );

    let mut iterations = 0;
    loop {
        push_button(&mut sim);
        iterations += 1;

        if sim.modules.get_mut("rx").unwrap().done() {
            break;
        }
    }

    Ok(iterations)
}

struct Simulation {
    modules: IndexMap<String, Box<dyn Module>>,
}

fn push_button(simulation: &mut Simulation) -> Vec<Pulse> {
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

        let module = simulation.modules.get_mut(&pulse.destination);
        if module.is_none() {
            // ignore unknown modules: they do not have any destinations
            continue;
        }
        let module = module.unwrap();

        let new_pulses = module.process(pulse);

        new_pulses
            .into_iter()
            .for_each(|pulse| pulse_queue.push_back(pulse));
    }

    processed_pulses
}

fn create_simulation(puzzle: Puzzle) -> Simulation {
    let modules = puzzle
        .modules
        .into_iter()
        .map(|cfg| {
            let name = cfg.name.clone();
            let enriched_cfg = EnrichedModuleConfig {
                module_config: cfg,
                inputs: puzzle.inputs.get(&name).unwrap_or(&vec![]).clone(),
            };
            let module: Box<dyn Module> = enriched_cfg.into();
            (name, module)
        })
        .collect();
    Simulation { modules }
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

trait Module: Debug {
    fn process(&mut self, pulse: Pulse) -> Vec<Pulse>;
    fn done(&self) -> bool {
        false
    }
}

#[derive(Debug)]
struct BaseModule {
    destinations: Vec<String>,
}
impl BaseModule {
    fn new_pulses_from(&self, high: bool, source: &str) -> Vec<Pulse> {
        self.destinations
            .iter()
            .map(|name| Pulse {
                destination: name.to_owned(),
                high,
                source: source.to_owned(),
            })
            .collect_vec()
    }
}

#[derive(Debug)]
struct BroadcastModule {
    base_module: BaseModule,
}
impl Module for BroadcastModule {
    fn process(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.base_module
            .new_pulses_from(pulse.high, &pulse.destination)
    }
}

#[derive(Debug)]
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
            self.base_module
                .new_pulses_from(self.on, &pulse.destination)
        }
    }
}

#[derive(Debug)]
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
        let low = self.last_received_from_was_high.values().all(|&high| high);
        self.base_module.new_pulses_from(!low, &pulse.destination)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::trim_lines;

    fn push_button_and_check(simulation: &mut Simulation, expected: &str) {
        let pulses = push_button(simulation);
        let actual = pulses.iter().map(ToString::to_string).join("\n");
        assert_eq!(actual, trim_lines(expected));
    }

    #[test]
    fn test_example1_step() {
        let data = day().read_data_file("example1").unwrap();
        let puzzle: Puzzle = data.parse().unwrap();
        let mut sim = create_simulation(puzzle);
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
        push_button_and_check(&mut sim, expected);
    }

    #[test]
    fn test_extra_example_steps() {
        let data = r"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output        
        ";
        let data = trim_lines(data);
        let puzzle: Puzzle = data.parse().unwrap();
        let mut sim = create_simulation(puzzle);

        // push button
        let expected = r"
            button -low-> broadcaster
            broadcaster -low-> a
            a -high-> inv
            a -high-> con
            inv -low-> b
            con -high-> output
            b -high-> con
            con -low-> output
            ";
        push_button_and_check(&mut sim, expected);

        // push button a second time
        let expected = r"
            button -low-> broadcaster
            broadcaster -low-> a
            a -low-> inv
            a -low-> con
            inv -high-> b
            con -high-> output
        ";
        push_button_and_check(&mut sim, expected);

        // push button a third time
        let expected = r"
            button -low-> broadcaster
            broadcaster -low-> a
            a -high-> inv
            a -high-> con
            inv -low-> b
            con -low-> output
            b -low-> con
            con -high-> output
        ";
        push_button_and_check(&mut sim, expected);

        // push button a fourth time
        let expected = r"
            button -low-> broadcaster
            broadcaster -low-> a
            a -low-> inv
            a -low-> con
            inv -high-> b
            con -high-> output
        ";
        push_button_and_check(&mut sim, expected);
    }

    #[test]
    fn test_part1_extra_example() {
        let data = r"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output        
        ";
        let data = trim_lines(data);
        assert_eq!(part1(&data).unwrap(), 11687500);
    }
}
