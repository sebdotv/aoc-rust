use crate::challenge::Day;
use anyhow::Result;
use indexmap::IndexMap;
use itertools::Itertools;
use std::cmp::Ordering;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (143, Some(5651)),
        part2_solutions: Some((123, Some(4743))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let (rules, updates) = parse_input(data);
    let mut sum = 0;
    for pages in updates {
        if is_sorted(&pages, &rules) {
            sum += pages.get(pages.len() / 2).unwrap();
        }
    }
    Ok(sum)
}

fn part2(data: &str) -> Result<usize> {
    let (rules, updates) = parse_input(data);
    let mut sum = 0;
    for pages in updates {
        if !is_sorted(&pages, &rules) {
            let pages = pages
                .into_iter()
                .sorted_by(rules.compare_fn())
                .collect_vec();
            sum += pages.get(pages.len() / 2).unwrap();
        }
    }
    Ok(sum)
}

fn parse_input(data: &str) -> (Rules, Vec<Vec<usize>>) {
    let lines = data.lines().collect_vec();
    let (rules, updates) = lines.split(|l| l.is_empty()).collect_tuple().unwrap();
    let rules: Rules = rules.into();
    let updates = updates
        .iter()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    (rules, updates)
}

fn is_sorted(pages: &[usize], rules: &Rules) -> bool {
    pages.is_sorted_by(rules.is_less_fn())
}

#[derive(Debug)]
struct Rules {
    ord: IndexMap<(usize, usize), Ordering>,
}
impl From<&[&str]> for Rules {
    fn from(value: &[&str]) -> Self {
        let mut ord = IndexMap::new();
        for line in value {
            let (a, b) = line
                .split('|')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            ord.insert((a, b), Ordering::Less);
            ord.insert((b, a), Ordering::Greater);
        }
        Rules { ord }
    }
}
impl Rules {
    fn compare(&self, a: usize, b: usize) -> Ordering {
        *self.ord.get(&(a, b)).unwrap()
    }

    const fn compare_fn(&self) -> impl FnMut(&usize, &usize) -> Ordering + '_ {
        |a, b| self.compare(*a, *b)
    }

    const fn is_less_fn(&self) -> impl FnMut(&usize, &usize) -> bool + '_ {
        |a, b| self.compare(*a, *b) == Ordering::Less
    }
}
