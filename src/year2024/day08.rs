use crate::challenge::Day;
use crate::utils::grid::{Coord, Grid};
use anyhow::Result;
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (14, Some(252)),
        part2_solutions: Some((34, Some(839))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    solve(data, Some((1, 1)))
}

fn part2(data: &str) -> Result<usize> {
    solve(data, None)
}

fn solve(data: &str, min_max: Option<(usize, usize)>) -> Result<usize> {
    let grid: Grid<char> = data.parse()?;
    let mut frequencies: IndexMap<char, Vec<Coord>> = IndexMap::new();
    for (coord, c) in &grid {
        if c != '.' {
            frequencies.entry(c).or_default().push(coord);
        }
    }

    let mut antinodes: IndexSet<Coord> = IndexSet::new();

    let mut try_antinode = |x: isize, y: isize| {
        let a = Coord(usize::try_from(x).ok()?, usize::try_from(y).ok()?);
        grid.maybe_get(&a)?;
        antinodes.insert(a);
        Some(())
    };

    for (_, coords) in &frequencies {
        #[allow(clippy::cast_possible_wrap)]
        for (Coord(x1, y1), Coord(x2, y2)) in coords.iter().tuple_combinations() {
            let dx = *x2 as isize - *x1 as isize;
            let dy = *y2 as isize - *y1 as isize;

            let mut explore_direction = |x: isize, y: isize, dir: isize| {
                let mut i = min_max.map_or(0, |(min, _)| min as isize * dir);
                loop {
                    let r = try_antinode(x + i * dx, y + i * dy);
                    if r.is_none()
                        || min_max
                            .map_or(false, |(_, max)| usize::try_from(i.abs()).unwrap() >= max)
                    {
                        break;
                    }
                    i += dir;
                }
            };

            explore_direction(*x1 as isize, *y1 as isize, -1);
            explore_direction(*x2 as isize, *y2 as isize, 1);
        }
    }
    Ok(antinodes.len())
}
