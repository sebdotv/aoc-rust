use anyhow::Result;
use itertools::Itertools;
use strum_macros::EnumString;

use crate::challenge::Day;
use crate::grid::Grid;

pub fn day() -> Day<i32> {
    Day {
        part1_solutions: (4, None),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<i32> {
    let grid: Grid<Tile> = Grid::from_lines(&data.lines().collect_vec());

    println!("{:?}", grid);

    // let start = grid.coords().find(|p|
    // grid.get()
    // )

    Ok(0)
}

#[derive(EnumString, Debug)]
enum Tile {
    #[strum(serialize = "|")]
    NS,
    #[strum(serialize = "-")]
    EW,
    #[strum(serialize = "L")]
    NE,
    #[strum(serialize = "J")]
    NW,
    #[strum(serialize = "7")]
    SW,
    #[strum(serialize = "F")]
    SE,
    #[strum(serialize = ".")]
    Ground,
    #[strum(serialize = "S")]
    Start,
}

fn part2(_data: &str) -> Result<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_extra_examples() -> Result<()> {
        let f = |s: &str| part1(s.trim().lines().map(str::trim).join("\n").as_str());
        assert_eq!(
            f(r"
                ..F7.
                .FJ|.
                SJ.L7
                |F--J
                LJ...
            ")?,
            8
        );
        Ok(())
    }
}
