use std::collections::HashMap;
use std::iter;
use std::str::FromStr;

use anyhow::{Context, Result};
use itertools::Itertools;
use strum_macros::{EnumIter, EnumString};

use crate::challenge::Day;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (21, Some(7716)),
        part2_solutions: Some((525152, Some(18716325559999))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let sum = data
        .lines()
        .map(|line| {
            let record: Record = line.parse()?;
            let arrangements = count_arrangements(&record);
            Ok(arrangements)
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum();
    Ok(sum)
}

fn part2(data: &str) -> Result<usize> {
    let sum = data
        .lines()
        .map(|line| {
            let line = expand(line);
            let record: Record = line.parse()?;
            let arrangements = count_arrangements(&record);
            Ok(arrangements)
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum();
    Ok(sum)
}

fn expand(line: &str) -> String {
    let (a, b) = line.split_once(' ').unwrap();
    [
        iter::repeat_n(a, 5).join("?"),
        iter::repeat_n(b, 5).join(","),
    ]
    .join(" ")
}

fn count_arrangements(record: &Record) -> usize {
    let mut cached_impl = Cached {
        cache: HashMap::new(),
        cache_stats: CacheStats { hit: 0, miss: 0 },
    };
    let result = cached_impl.rec(
        record.pattern.as_slice(),
        record.groups.as_slice(),
        Remaining::Free,
    );
    result
}

#[derive(Debug, Copy, Clone)]
enum Remaining {
    Damaged(usize),
    NonDamaged,
    Free,
}
impl Remaining {
    fn for_damaged(value: usize) -> Self {
        if value > 0 {
            Self::Damaged(value)
        } else {
            Self::NonDamaged
        }
    }
}

#[derive(Eq, PartialEq)]
enum Outcome {
    Zero,
    Skip,
    UseRemaining,
    UseNextGroup,
    UseNextGroupOrSkip,
}

#[derive(Debug)]
struct CacheStats {
    hit: usize,
    miss: usize,
}
struct Cached {
    cache: HashMap<String, usize>,
    cache_stats: CacheStats,
}
impl Cached {
    fn rec(&mut self, pattern: &[Condition], groups: &[usize], remaining: Remaining) -> usize {
        let key = format!(
            "{}, {:?}, {:?}",
            pattern.iter().map(ToString::to_string).join(""),
            groups,
            remaining
        );

        if let Some(value) = self.cache.get(&key) {
            self.cache_stats.hit += 1;
            *value
        } else {
            self.cache_stats.miss += 1;
            let value = self.it(pattern, groups, remaining);
            self.cache.insert(key, value);
            value
        }
    }

    fn it(&mut self, pattern: &[Condition], groups: &[usize], remaining: Remaining) -> usize {
        use Condition::*;
        use Outcome::*;

        if pattern.is_empty() {
            return match remaining {
                Remaining::NonDamaged | Remaining::Free if groups.is_empty() => 1,
                _ => 0,
            };
        }

        let outcome: Outcome = match pattern[0] {
            Operational => match remaining {
                Remaining::Damaged(_) => Zero, // wanted Damaged but got Operational
                Remaining::NonDamaged | Remaining::Free => Skip,
            },
            Damaged => match remaining {
                Remaining::Damaged(_) => UseRemaining,
                Remaining::NonDamaged => Zero, // wanted NonDamaged but got Damaged
                Remaining::Free => UseNextGroup,
            },
            Unknown => match remaining {
                Remaining::Damaged(_) => UseRemaining,
                Remaining::NonDamaged => Skip,
                Remaining::Free => {
                    if groups.is_empty() {
                        Skip
                    } else {
                        UseNextGroupOrSkip
                    }
                }
            },
        };

        let next_pattern = &pattern[1..];
        match outcome {
            Zero => 0,
            UseRemaining => self.rec(
                next_pattern,
                groups,
                match remaining {
                    Remaining::Damaged(remaining) => Remaining::for_damaged(remaining - 1),
                    _ => unreachable!(),
                },
            ),
            Skip | UseNextGroup | UseNextGroupOrSkip => {
                let a = if outcome == UseNextGroup || outcome == UseNextGroupOrSkip {
                    if groups.is_empty() {
                        0
                    } else {
                        self.rec(
                            next_pattern,
                            &groups[1..],
                            Remaining::for_damaged(groups[0] - 1),
                        )
                    }
                } else {
                    0
                };
                let b = if outcome == Skip || outcome == UseNextGroupOrSkip {
                    self.rec(next_pattern, groups, Remaining::Free)
                } else {
                    0
                };
                a + b
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Record {
    pattern: Vec<Condition>,
    groups: Vec<usize>,
}

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (pattern, groups) = s
            .split_once(' ')
            .ok_or_else(|| anyhow::anyhow!("missing space"))?;
        let pattern = pattern
            .chars()
            .map(|c| {
                c.to_string()
                    .parse()
                    .with_context(|| format!("failed to parse condition: {}", c))
            })
            .collect::<Result<_>>()?;
        let groups = groups
            .split(',')
            .map(|s| {
                s.parse()
                    .with_context(|| format!("failed to parse group: {}", s))
            })
            .collect::<Result<_>>()?;
        Ok(Self { pattern, groups })
    }
}

#[derive(EnumString, Debug, Eq, PartialEq, strum_macros::Display, Copy, Clone, EnumIter)]
enum Condition {
    #[strum(serialize = ".")]
    Operational,
    #[strum(serialize = "#")]
    Damaged,
    #[strum(serialize = "?")]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let test_vectors = vec![
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 4),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 1),
            ("????.######..#####. 1,6,5", 4),
            ("?###???????? 3,2,1", 10),
        ];

        for (line, expected) in test_vectors {
            let record = line.parse().unwrap();
            let arrangements = count_arrangements(&record);
            assert_eq!(arrangements, expected);
        }
    }

    #[test]
    fn part2_examples() {
        let test_vectors = vec![
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 16384),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 16),
            ("????.######..#####. 1,6,5", 2500),
            ("?###???????? 3,2,1", 506250),
        ];

        for (line, expected) in test_vectors {
            let line = expand(line);
            let record = line.parse().unwrap();
            let arrangements = count_arrangements(&record);
            assert_eq!(arrangements, expected);
        }
    }
}
