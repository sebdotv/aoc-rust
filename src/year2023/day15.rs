use anyhow::Result;

use crate::challenge::Day;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (1320, Some(515210)),
        part2_solutions: None,
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

#[allow(clippy::unnecessary_wraps)]
fn part1(data: &str) -> Result<usize> {
    Ok(data
        .trim()
        .split(',')
        .map(hash)
        .map(|h| TryInto::<usize>::try_into(h).unwrap())
        .sum())
}

fn part2(_data: &str) -> Result<usize> {
    todo!()
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
