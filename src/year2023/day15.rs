use anyhow::Result;
use indexmap::IndexMap;
use std::iter;
use std::str::FromStr;

use crate::challenge::Day;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (1320, Some(515210)),
        part2_solutions: Some((145, Some(246762))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn hash(s: &str) -> u8 {
    let mut h: usize = 0;
    for c in s.chars() {
        h += c as usize;
        h *= 17;
        h %= 256;
    }
    h.try_into().unwrap()
}

fn part1(data: &str) -> Result<usize> {
    Ok(data
        .trim()
        .split(',')
        .map(hash)
        .map(Into::<usize>::into)
        .sum())
}

fn part2(data: &str) -> Result<usize> {
    let steps = data
        .trim()
        .split(',')
        .map(str::parse::<Step>)
        .collect::<Result<Vec<_>>>()?;

    let mut boxes: Vec<IndexMap<String, usize>> =
        iter::repeat_with(IndexMap::new).take(256).collect();

    for step in steps {
        let box_id = hash(&step.label) as usize;

        let target_box = &mut boxes[box_id];

        match step.operation {
            Operation::Remove => {
                target_box.shift_remove(&step.label);
            }
            Operation::Add(focal) => {
                target_box.insert(step.label, focal);
            }
        }
    }

    let sum = boxes
        .iter()
        .enumerate()
        .map(|(box_id, b)| {
            let box_sum = b
                .values()
                .enumerate()
                .map(|(lens_slot, focal_length)| (1 + box_id) * (1 + lens_slot) * focal_length)
                .sum::<usize>();
            box_sum
        })
        .sum::<usize>();

    Ok(sum)
}

#[derive(Debug)]
struct Step {
    label: String,
    operation: Operation,
}
#[derive(Debug)]
enum Operation {
    Remove,
    Add(usize),
}
impl FromStr for Step {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let idx = s.find(|c| c == '-' || c == '=').unwrap();
        let (label, rest) = s.split_at(idx);
        let (op, nb) = rest.split_at(1);
        let label = label.to_string();
        let operation = match op {
            "-" => Operation::Remove,
            "=" => Operation::Add(nb.parse::<usize>()?),
            _ => anyhow::bail!("Invalid operation: {} in {}", op, s),
        };
        Ok(Step { label, operation })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_example() {
        assert_eq!(hash("HASH"), 52);

        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }
}
