use std::ops::Range;
use std::str::FromStr;

use anyhow::Result;
use indexmap::IndexMap;
use itertools::Itertools;
use strum_macros::EnumString;

use crate::challenge::Day;
use crate::year2023::day05::Category::{Location, Seed};

pub fn day() -> Day<u64> {
    Day {
        part1_solutions: (35, Some(535088217)),
        part2_solutions: Some((46, Some(51399228))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<u64> {
    let puzzle = data.parse::<Puzzle>()?;
    let min = puzzle
        .seeds
        .iter()
        .map(|seeds| {
            let item = (*seeds, Seed);
            let (number, category) = puzzle.resolve_recursively(item);
            assert_eq!(category, Location);
            number
        })
        .min()
        .unwrap();
    Ok(min)
}

fn part2(data: &str) -> Result<u64> {
    let puzzle = data.parse::<Puzzle>()?;
    let seed_ranges = puzzle
        .seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.collect_tuple::<(_, _)>().unwrap())
        .map(|(start, len)| *start..*start + *len)
        .collect_vec();

    let mut category = &Seed;
    let mut ranges = seed_ranges.clone();

    loop {
        let map = puzzle.maps_by_source.get(category);
        if map.is_none() {
            break;
        }
        let map = map.unwrap();

        let destination_ranges = ranges
            .iter()
            .flat_map(|range| map.transform(range))
            .collect_vec();

        category = &map.destination;
        ranges = destination_ranges;
    }

    assert_eq!(category, &Location);

    let min = ranges.iter().map(|range| range.start).min().unwrap();
    Ok(min)
}

#[derive(Debug)]
struct Puzzle {
    seeds: Vec<u64>,
    maps_by_source: IndexMap<Category, ConversionMap>,
}

type Item = (u64, Category);

impl Puzzle {
    pub fn resolve(&self, item: Item) -> Option<Item> {
        let (number, category) = item;
        let map = self.maps_by_source.get(&category)?;
        let range = map.ranges.iter().find(|range| {
            range.source_start <= number && number < range.source_start + range.length
        });
        let destination_number = range.map_or(number, |range| {
            range.destination_start + number - range.source_start
        });
        Some((destination_number, map.destination.clone()))
    }

    pub fn resolve_recursively(&self, item: Item) -> Item {
        match self.resolve(item.clone()) {
            Some(resolved) => self.resolve_recursively(resolved),
            None => item,
        }
    }

    fn new(seeds: Vec<u64>, maps: Vec<ConversionMap>) -> Puzzle {
        let len = maps.len();
        let maps_by_source: IndexMap<Category, ConversionMap> = maps
            .into_iter()
            .map(|map| (map.source.clone(), map))
            .collect();
        assert_eq!(maps_by_source.len(), len);
        Puzzle {
            seeds,
            maps_by_source,
        }
    }
}

impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        let paragraphs = lines.split(|line| line.is_empty()).collect_vec();
        let (seeds, maps) = paragraphs.split_first().unwrap();

        let (seeds,) = seeds.iter().collect_tuple().unwrap();
        let seeds = seeds.strip_prefix("seeds: ").unwrap();
        let seeds = seeds
            .split(' ')
            .map(|part| part.parse::<u64>().unwrap())
            .collect_vec();

        let maps = maps
            .iter()
            .map(|paragraph| paragraph.join("\n").parse::<ConversionMap>().unwrap())
            .collect_vec();

        Ok(Puzzle::new(seeds, maps))
    }
}

#[derive(Debug)]
struct ConversionMap {
    source: Category,
    destination: Category,
    ranges: Vec<ConversionRange>, // sorted by source_start
}

impl ConversionMap {
    pub fn transform(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        let conversions = self
            .ranges
            .iter()
            .filter_map(|r| r.transform(range).map(|transformed| (r, transformed)))
            .collect_vec();

        let mut ranges = vec![];
        let mut pos = range.start;

        for (conv, dest_range) in conversions {
            let source_range = conv.source_range();

            if let Some(r) = maybe_range(pos, source_range.start) {
                ranges.push(r);
            }

            ranges.push(dest_range.clone());
            pos = source_range.end;
        }

        if let Some(r) = maybe_range(pos, range.end) {
            ranges.push(r);
        }

        ranges
    }
}

fn maybe_range(start: u64, end: u64) -> Option<Range<u64>> {
    if start < end {
        Some(start..end)
    } else {
        None
    }
}

impl FromStr for ConversionMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        let (first, ranges) = lines.split_first().unwrap();

        let first = first.strip_suffix(" map:").unwrap();

        let categories: [&str; 2] = first.split_once("-to-").unwrap().into();
        let categories = categories.map(|category| category.parse::<Category>().unwrap());
        let [source, destination] = categories;

        Ok(ConversionMap {
            source,
            destination,
            ranges: ranges
                .iter()
                .map(|line| line.parse::<ConversionRange>().unwrap())
                .sorted_by_key(|r| r.source_start)
                .collect_vec(),
        })
    }
}

#[derive(Debug)]
struct ConversionRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl ConversionRange {
    pub fn source_range(&self) -> Range<u64> {
        self.source_start..self.source_start + self.length
    }

    pub fn transform(&self, range: &Range<u64>) -> Option<Range<u64>> {
        let intersection_start = range.start.max(self.source_start);
        let intersection_end = range.end.min(self.source_start + self.length);
        if intersection_start >= intersection_end {
            None
        } else {
            let diff = i128::from(self.destination_start) - i128::from(self.source_start);
            let start = u64::try_from(i128::from(intersection_start) + diff).unwrap();
            let end = u64::try_from(i128::from(intersection_end) + diff).unwrap();
            Some(start..end)
        }
    }
}

impl FromStr for ConversionRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (destination_start, source_start, length) = s
            .split(' ')
            .map(|part| part.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(ConversionRange {
            destination_start,
            source_start,
            length,
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, EnumString)]
#[strum(serialize_all = "lowercase")]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_works() {
        let cr = ConversionRange {
            destination_start: 0,
            source_start: 1147,
            length: 444,
        };
        let r: Range<u64> = 1115..1195;
        let transformed = cr.transform(&r).unwrap();
        assert_eq!(transformed, 0..48);
    }
}
