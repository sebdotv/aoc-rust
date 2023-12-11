use anyhow::Result;
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

use crate::challenge::Day;
use crate::grid::{Coord, Direction, Grid};

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (4, Some(6846)),
        part2_solutions: Some((4, Some(325))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: true,
    }
}

fn part1(data: &str) -> Result<usize> {
    let lines = data.lines().collect_vec();
    let grid: Grid<Tile> = Grid::from_lines(&lines)?;

    let solver = Solver { grid };

    let start = solver.find_start();

    let (_, (a, b)) = solver.find_firsts(start);

    let dist_a = solver.explore(start, &a);
    let dist_b = solver.explore(start, &b);

    assert_eq!(dist_a.keys().len(), dist_b.keys().len());

    let max_dist = dist_a
        .iter()
        .map(|(c, dist_a)| {
            let dist_b = dist_b.get(c).unwrap();
            dist_a.min(dist_b)
        })
        .max()
        .unwrap();

    Ok(*max_dist)
}

fn part2_internal(data: &str) -> Result<(Solver, IndexSet<Coord>, Vec<Coord>)> {
    let lines = data.lines().collect_vec();
    let grid: Grid<Tile> = Grid::from_lines(&lines)?;

    let solver = Solver { grid };
    let start = solver.find_start();
    let (start_tile, (first, _)) = solver.find_firsts(start);
    let explored = solver.explore(start, &first);
    let mut loop_coords: IndexSet<_> = explored.into_keys().collect();

    loop_coords.insert(start);

    let final_solver = solver.transform_grid(|(_, &tile)| {
        use Tile::*;
        match tile {
            Start => start_tile,
            _ => tile,
        }
    });

    let enclosed = final_solver.find_enclosed(&loop_coords);

    Ok((solver, loop_coords, enclosed))
}
fn part2(data: &str) -> Result<usize> {
    let (_, _, enclosed) = part2_internal(data)?;
    Ok(enclosed.len())
}

struct Solver {
    grid: Grid<Tile>,
}

impl Solver {
    pub fn transform_grid<F>(&self, f: F) -> Solver
    where
        F: Fn((&Coord, &Tile)) -> Tile,
    {
        let grid = self.grid.transform(f);
        Solver { grid }
    }
}

type Flow = (Coord, Direction);
impl Solver {
    fn find_start(&self) -> Coord {
        let (start,) = self
            .grid
            .coords()
            .filter(|p| *self.grid.get(p) == Tile::Start)
            .collect_tuple()
            .unwrap();
        start
    }

    fn find_firsts(&self, start: Coord) -> (Tile, (Flow, Flow)) {
        let ((dir_a, a), (dir_b, b)) = Direction::iter()
            .filter_map(|d| self.find_next(start, d).map(|flow| (d, flow)))
            .collect_tuple()
            .unwrap();
        let connections = IndexSet::from([dir_a, dir_b]);
        let (start_tile,) = Tile::iter()
            .filter(|t| t.connections() == connections)
            .collect_tuple()
            .unwrap();
        (start_tile, (a, b))
    }

    fn find_next(&self, pos: Coord, dir: Direction) -> Option<Flow> {
        self.grid.walk(&pos, dir).and_then(|neighbor| {
            let neighbor_tile = self.grid.get(&neighbor);
            let next_dir = neighbor_tile.direction_from(dir);
            next_dir.map(|d| (neighbor, d))
        })
    }

    fn explore(&self, start: Coord, first: &Flow) -> IndexMap<Coord, usize> {
        let mut distances: IndexMap<Coord, usize> = IndexMap::new();

        let mut current = *first;
        let mut dist = 1;

        loop {
            let (pos, dir) = current;

            if pos == start {
                break;
            }

            let prev = distances.insert(pos, dist);
            assert!(prev.is_none());

            let next = self.find_next(pos, dir);
            if next.is_none() {
                break;
            }

            current = next.unwrap();

            dist += 1;
        }

        distances
    }

    fn find_enclosed(&self, loop_coords: &IndexSet<Coord>) -> Vec<Coord> {
        use Direction::*;
        let vertical_directions = &IndexSet::from([N, S]);

        self.grid
            .row_coords()
            .into_iter()
            .flat_map(|row_coords| {
                let mut inside = false;
                let mut pending_crossing: Option<Direction> = None;
                row_coords.into_iter().filter(move |coord| {
                    if loop_coords.contains(coord) {
                        let connections = self.grid.get(coord).connections();

                        let vertical_connections: IndexSet<_> =
                            connections.intersection(vertical_directions).collect();

                        let crossing =
                            match vertical_connections.into_iter().collect_vec().as_slice() {
                                [_, _] => {
                                    pending_crossing = None;
                                    true
                                }
                                [vertical_direction] => {
                                    if let Some(prev_partial) = pending_crossing {
                                        let crossing = prev_partial != **vertical_direction;
                                        pending_crossing = None;
                                        crossing
                                    } else {
                                        pending_crossing = Some(**vertical_direction);
                                        false
                                    }
                                }
                                [] => false,
                                _ => unreachable!(),
                            };

                        if crossing {
                            inside = !inside;
                        }
                        false
                    } else {
                        inside
                    }
                })
            })
            .collect()
    }

    #[allow(dead_code)]
    fn grid_to_string(
        &self,
        loop_coords: &IndexSet<Coord>,
        enclosed: &IndexSet<Coord>,
        keep_non_enclosed: bool,
    ) -> String {
        self.grid
            .transform(|(coord, tile)| {
                if loop_coords.contains(coord) {
                    tile.to_string()
                } else if enclosed.contains(coord) {
                    "I".to_owned()
                } else if keep_non_enclosed {
                    tile.to_string()
                } else {
                    "O".to_owned()
                }
            })
            .to_string()
    }
}

#[derive(EnumString, Debug, Eq, PartialEq, strum_macros::Display, Copy, Clone, EnumIter)]
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

impl Tile {
    fn connections(self) -> IndexSet<Direction> {
        use Direction::*;
        use Tile::*;
        match self {
            NS => IndexSet::from([N, S]),
            EW => IndexSet::from([E, W]),
            NE => IndexSet::from([N, E]),
            NW => IndexSet::from([N, W]),
            SW => IndexSet::from([S, W]),
            SE => IndexSet::from([S, E]),
            Ground | Start => IndexSet::new(),
        }
    }

    fn direction_from(self, dir: Direction) -> Option<Direction> {
        let mut connections = self.connections().clone();
        let removed = connections.remove(&dir.reverse());
        removed.then(|| {
            let (other_dir,) = connections.iter().collect_tuple().unwrap();
            *other_dir
        })
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::testing::trim_lines;

    use super::*;

    #[test]
    fn part1_extra_example() -> Result<()> {
        let f = |s: &str| part1(&trim_lines(s));
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
    #[test]
    fn part2_extra_examples() -> Result<()> {
        let f_common = |s: &str, keep_non_enclosed: bool| {
            part2_internal(s).map(|(solver, loop_coords, enclosed)| {
                let enclosed = IndexSet::from_iter(enclosed);
                (
                    enclosed.len(),
                    solver.grid_to_string(&loop_coords, &enclosed, keep_non_enclosed),
                )
            })
        };
        let f = |s: &str| f_common(s, false);
        let f_keep = |s: &str| f_common(s, true);
        assert_eq!(
            f(indoc! {"
                .F----7F7F7F7F-7....
                .|F--7||||||||FJ....
                .||.FJ||||||||L7....
                FJL7L7LJLJ||LJ.L-7..
                L--J.L7...LJS7F-7L7.
                ....F-J..F7FJ|L7L7L7
                ....L7.F7||L7|.L7L7|
                .....|FJLJ|FJ|F7|.LJ
                ....FJL-7.||.||||...
                ....L---J.LJ.LJLJ...
            "})?,
            (
                8,
                indoc! {"
                    OF----7F7F7F7F-7OOOO
                    O|F--7||||||||FJOOOO
                    O||OFJ||||||||L7OOOO
                    FJL7L7LJLJ||LJIL-7OO
                    L--JOL7IIILJS7F-7L7O
                    OOOOF-JIIF7FJ|L7L7L7
                    OOOOL7IF7||L7|IL7L7|
                    OOOOO|FJLJ|FJ|F7|OLJ
                    OOOOFJL-7O||O||||OOO
                    OOOOL---JOLJOLJLJOOO
                    "}
                .to_owned()
            )
        );
        assert_eq!(
            f_keep(indoc! {"
                FF7FSF7F7F7F7F7F---7
                L|LJ||||||||||||F--J
                FL-7LJLJ||||||LJL-77
                F--JF--7||LJLJ7F7FJ-
                L---JF-JLJ.||-FJLJJ7
                |F|F-JF---7F7-L7L|7|
                |FFJF7L7F-JF7|JL---7
                7-L-JL7||F7|L7F-7F7|
                L.L7LFJ|||||FJL7||LJ
                L7JLJL-JLJLJL--JLJ.L
            "})?,
            (
                10,
                indoc! {"
                    FF7FSF7F7F7F7F7F---7
                    L|LJ||||||||||||F--J
                    FL-7LJLJ||||||LJL-77
                    F--JF--7||LJLJIF7FJ-
                    L---JF-JLJIIIIFJLJJ7
                    |F|F-JF---7IIIL7L|7|
                    |FFJF7L7F-JF7IIL---7
                    7-L-JL7||F7|L7F-7F7|
                    L.L7LFJ|||||FJL7||LJ
                    L7JLJL-JLJLJL--JLJ.L
                "}
                .to_owned()
            )
        );
        Ok(())
    }
}
