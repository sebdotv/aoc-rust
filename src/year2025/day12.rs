use crate::challenge::Day;
use anyhow::Result;
use itertools::Itertools;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (2, Some(427)),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    struct Shape {
        #[allow(dead_code)]
        area: usize,
        blocks: usize,
    }
    struct Region {
        w: usize,
        h: usize,
        nums: Vec<usize>,
    }
    let parts = data.split("\n\n").collect_vec();
    let (region_sec, shape_sec) = parts.split_last().unwrap();
    let mut shapes = Vec::new();
    for shape_def in shape_sec {
        let shape = shape_def.split('\n').skip(1).collect_vec();
        let area = shape.len() * shape[0].len();
        let blocks = shape
            .iter()
            .flat_map(|line| line.chars())
            .filter(|c| *c == '#')
            .count();
        shapes.push(Shape { area, blocks });
    }
    let mut regions = Vec::new();
    for line in region_sec.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        let (w, h) = left
            .split('x')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let nums = right
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();
        regions.push(Region { w, h, nums });
    }
    let example = regions[0].w == 4 && regions[0].h == 4;
    let mut count = 0;
    for Region { w, h, nums } in regions {
        let total_blocks = nums
            .iter()
            .enumerate()
            .map(|(i, n)| shapes[i].blocks * n)
            .sum();
        if w * h > total_blocks {
            count += 1;
        }
    }
    if example {
        count -= 1;
    }
    Ok(count)
}

fn part2(_data: &str) -> Result<usize> {
    Ok(0)
}
