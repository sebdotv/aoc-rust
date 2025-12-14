use crate::challenge::Day;
use crate::utils::point3::Point3;
use anyhow::{Result, bail};
use itertools::Itertools;
use std::collections::HashMap;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (40, Some(129564)),
        part2_solutions: Some((25272, Some(42047840))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let points = parse_input(data);
    let example = points.first().unwrap().x == 162;
    let mut pairs = points.iter().tuple_combinations().collect_vec();
    pairs.sort_by_key(|(p1, p2)| p1.dist2(p2));

    let steps = if example { 10 } else { 1000 };
    let mut state = State::new(points.clone());
    for (a, b) in pairs.into_iter().take(steps) {
        state.connect(a, b);
    }

    let (circuits, _) = state.get_circuits();
    let mut circuit_sizes = circuits.values().map(Vec::len).collect_vec();
    circuit_sizes.sort_unstable();
    circuit_sizes.reverse();
    let product = circuit_sizes.iter().take(3).product();

    Ok(product)
}
fn part2(data: &str) -> Result<usize> {
    let points = parse_input(data);
    let mut pairs = points.iter().tuple_combinations().collect_vec();
    pairs.sort_by_key(|(p1, p2)| p1.dist2(p2));

    let mut state = State::new(points.clone());
    for (a, b) in pairs {
        state.connect(a, b);
        let (circuits, unconnected) = state.get_circuits();
        if unconnected.is_empty() && circuits.len() == 1 {
            return Ok(usize::try_from(a.x * b.x)?);
        }
    }
    bail!("no solution")
}

type Scalar = i64;
type Point = Point3<Scalar>;

struct State {
    points: Vec<Point>,
    next_circuit_id: usize,
    circuits: HashMap<Point, usize>,
}
impl State {
    fn new(points: Vec<Point>) -> Self {
        Self {
            // tree,
            points,
            next_circuit_id: 0,
            circuits: HashMap::default(),
        }
    }

    fn connect(&mut self, a: &Point, b: &Point) {
        let circuit_a = self.circuits.get(a);
        let (circuit_id, merged_circuit_id) = match (circuit_a, self.circuits.get(b)) {
            (Some(c_a), Some(c_b)) => {
                // move whole circuit of b to a
                (*c_a, Some(*c_b))
            }
            (None, None) => {
                let circuit_id = self.next_circuit_id;
                self.next_circuit_id += 1;
                (circuit_id, None)
            }
            (Some(circuit_id), None) | (None, Some(circuit_id)) => (*circuit_id, None),
        };
        if let Some(merged_circuit_id) = merged_circuit_id {
            #[allow(clippy::iter_over_hash_type)] // iteration order does not matter here
            for cid in self.circuits.values_mut() {
                if *cid == merged_circuit_id {
                    *cid = circuit_id;
                }
            }
        }
        self.assign_circuit_id(a, circuit_id);
        self.assign_circuit_id(b, circuit_id);
    }
    fn assign_circuit_id(&mut self, p: &Point, circuit_id: usize) {
        let prev = self.circuits.insert(*p, circuit_id);
        assert!(prev.is_none() || prev == Some(circuit_id));
    }
    fn get_circuits(&self) -> (HashMap<usize, Vec<&Point>>, Vec<&Point>) {
        let mut circuits = HashMap::new();
        let mut unconnected = vec![];
        for p in &self.points {
            let dest = if let Some(cid) = self.circuits.get(p) {
                circuits.entry(*cid).or_default()
            } else {
                &mut unconnected
            };
            dest.push(p);
        }
        (circuits, unconnected)
    }
}

fn parse_input(data: &str) -> Vec<Point> {
    let mut points = vec![];
    for line in data.lines() {
        let (x, y, z) = line
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        points.push(Point { x, y, z });
    }
    points
}
