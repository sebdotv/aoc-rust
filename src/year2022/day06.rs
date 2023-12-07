use anyhow::{anyhow, Error, Result};
use indexmap::IndexSet;
use itertools::Itertools;

use crate::challenge::Day;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (7, Some(1723)),
        part2_solutions: Some((19, Some(3708))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    find_marker(data, 4)
}
fn part2(data: &str) -> Result<usize> {
    find_marker(data, 14)
}

fn find_marker(data: &str, marker_len: usize) -> Result<usize, Error> {
    data.chars()
        .enumerate()
        .collect_vec()
        .windows(marker_len)
        .find_map(|w| {
            let unique_chars: IndexSet<_> = w.iter().map(|(_, c)| c).collect();
            let len = unique_chars.len();
            (len == marker_len).then(|| w[0].0 + len)
        })
        .ok_or(anyhow!("Could not find solution"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_extra_examples() -> Result<()> {
        let f = part1;
        assert_eq!(f("bvwbjplbgvbhsrlpgdmjqwftvncz")?, 5);
        assert_eq!(f("nppdvjthqldpwncqszvftbrmjlhg")?, 6);
        assert_eq!(f("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 10);
        assert_eq!(f("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 11);
        Ok(())
    }
    #[test]
    fn part2_extra_examples() -> Result<()> {
        let f = part2;
        assert_eq!(f("bvwbjplbgvbhsrlpgdmjqwftvncz")?, 23);
        assert_eq!(f("nppdvjthqldpwncqszvftbrmjlhg")?, 23);
        assert_eq!(f("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 29);
        assert_eq!(f("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 26);
        Ok(())
    }
}
