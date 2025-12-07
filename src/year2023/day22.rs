use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::{Result, bail};
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;

use crate::challenge::Day;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (5, Some(477)),
        part2_solutions: Some((7, Some(61555))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let CommonPart {
        grid,
        supporters,
        supporting,
    } = common_part(data)?;

    let mut removable = 0;

    for id in grid.bricks.keys() {
        let can_be_removed = if let Some(supporting) = supporting.get(id) {
            supporting.iter().all(|supported| {
                let supporters = supporters.get(supported).unwrap();
                supporters.len() > 1
            })
        } else {
            true
        };
        if can_be_removed {
            removable += 1;
        }
    }

    Ok(removable)
}

fn part2(data: &str) -> Result<usize> {
    #[derive(Clone)]
    struct State {
        supporters: IndexMap<BrickId, IndexSet<BrickId>>,
        supporting: IndexMap<BrickId, IndexSet<BrickId>>,
    }

    let CommonPart {
        grid,
        supporters,
        supporting,
    } = common_part(data)?;

    let to_set_values = |m: IndexMap<BrickId, Vec<BrickId>>| {
        m.into_iter()
            .map(|(k, v)| (k, IndexSet::from_iter(v)))
            .collect()
    };

    let state = State {
        supporters: to_set_values(supporters),
        supporting: to_set_values(supporting),
    };

    let mut sum = 0;

    for id in grid.bricks.keys() {
        let mut state = state.clone();
        let mut remove_queue: Vec<BrickId> = vec![];
        remove_queue.push(id.clone());
        let mut removed = 0;

        while let Some(to_remove) = remove_queue.pop() {
            let supporting = state.supporting.swap_remove(&to_remove).unwrap_or_default();
            for supported in supporting {
                let supporters = state.supporters.get_mut(&supported).unwrap();
                let removed = supporters.swap_remove(&to_remove);
                assert!(removed);
                if supporters.is_empty() {
                    remove_queue.push(supported);
                }
            }
            removed += 1;
        }

        sum += removed - 1; // ignore self-removal
    }

    Ok(sum)
}

struct CommonPart {
    grid: BrickGrid,
    supporters: IndexMap<BrickId, Vec<BrickId>>,
    supporting: IndexMap<BrickId, Vec<BrickId>>,
}

fn common_part(data: &str) -> Result<CommonPart> {
    let bricks = data
        .lines()
        .map(Brick::from_str)
        .collect::<Result<Vec<_>>>()?;

    let mut grid = BrickGrid::from(bricks);

    let bricks_asc: Vec<BrickId> = grid
        .bricks
        .iter()
        .sorted_by_key(|(_, brick)| brick.min_z())
        .map(|(id, _)| id)
        .cloned()
        .collect_vec();
    for id in bricks_asc {
        loop {
            let brick = grid.bricks.get(&id).unwrap();
            let z = brick.min_z();
            if z == 1 {
                break;
            }

            let xys_at_z = brick.xys_at_z(z);

            let can_move = if let Some(x) = grid.z.get(&(z - 1)) {
                !xys_at_z.iter().any(|xy| x.contains_key(xy))
            } else {
                true
            };

            if !can_move {
                break;
            }

            grid.replace_brick(&id, brick.clone().move_down());
        }
    }

    let supporters: IndexMap<BrickId, Vec<BrickId>> = grid
        .bricks
        .keys()
        .map(|id| {
            (
                id.clone(),
                grid.supporters(id).into_iter().cloned().collect(),
            )
        })
        .collect();

    let mut supporting: IndexMap<BrickId, Vec<BrickId>> = IndexMap::new();
    for id in grid.bricks.keys() {
        for supporter in grid.supporters(id) {
            supporting
                .entry(supporter.clone())
                .or_default()
                .push(id.clone());
        }
    }

    Ok(CommonPart {
        grid,
        supporters,
        supporting,
    })
}

#[derive(Debug, Clone)]
enum Brick {
    Cube {
        x: usize,
        y: usize,
        z: usize,
    },
    XLine {
        x: RangeInclusive<usize>,
        y: usize,
        z: usize,
    },
    YLine {
        x: usize,
        y: RangeInclusive<usize>,
        z: usize,
    },
    ZLine {
        x: usize,
        y: usize,
        z: RangeInclusive<usize>,
    },
}

impl Brick {
    fn min_z(&self) -> usize {
        use Brick::*;
        match self {
            Cube { z, .. } | XLine { z, .. } | YLine { z, .. } => *z,
            ZLine { z, .. } => *z.start(),
        }
    }
    fn coords(&self) -> Vec<Coord3D> {
        use Brick::*;
        match self {
            Cube { x, y, z } => vec![(*x, *y, *z)],
            XLine { x, y, z } => x.clone().map(|x| (x, *y, *z)).collect(),
            YLine { x, y, z } => y.clone().map(|y| (*x, y, *z)).collect(),
            ZLine { x, y, z } => z.clone().map(|z| (*x, *y, z)).collect(),
        }
    }
    fn xys_at_z(&self, at_z: usize) -> Vec<(usize, usize)> {
        use Brick::*;
        match self {
            Cube { x, y, z } if *z == at_z => vec![(*x, *y)],
            XLine { x, y, z } if *z == at_z => x.clone().map(|x| (x, *y)).collect(),
            YLine { x, y, z } if *z == at_z => y.clone().map(|y| (*x, y)).collect(),
            ZLine { x, y, z } if z.contains(&at_z) => vec![(*x, *y)],
            _ => vec![],
        }
    }
    pub fn move_down(self) -> Self {
        use Brick::*;
        match self {
            Cube { x, y, z } => Cube { x, y, z: z - 1 },
            XLine { x, y, z } => XLine { x, y, z: z - 1 },
            YLine { x, y, z } => YLine { x, y, z: z - 1 },
            ZLine { x, y, z } => ZLine {
                x,
                y,
                #[allow(clippy::range_minus_one)]
                z: (z.start() - 1)..=(z.end() - 1),
            },
        }
    }
}

impl FromStr for Brick {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Brick::*;
        let (start, end) = s.split_once('~').unwrap();
        let parse_xyz = |s: &str| {
            let (x, y, z) = s
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            (x, y, z)
        };
        let (start_x, start_y, start_z) = parse_xyz(start);
        let (end_x, end_y, end_z) = parse_xyz(end);
        let (len_x, len_y, len_z) = (end_x - start_x, end_y - start_y, end_z - start_z);
        let brick = match (len_x, len_y, len_z) {
            (0, 0, 0) => Cube {
                x: start_x,
                y: start_y,
                z: start_z,
            },
            (x, 0, 0) if x > 0 => XLine {
                x: start_x..=end_x,
                y: start_y,
                z: start_z,
            },
            (0, y, 0) if y > 0 => YLine {
                x: start_x,
                y: start_y..=end_y,
                z: start_z,
            },
            (0, 0, z) if z > 0 => ZLine {
                x: start_x,
                y: start_y,
                z: start_z..=end_z,
            },
            _ => bail!("Invalid brick: {}", s),
        };
        Ok(brick)
    }
}

type Coord3D = (usize, usize, usize);
type BrickId = String;

struct BrickGrid {
    bricks: IndexMap<BrickId, Brick>,
    // z -> (x,y) -> brick_id
    z: IndexMap<usize, IndexMap<(usize, usize), BrickId>>,
}

impl BrickGrid {
    fn new() -> Self {
        Self {
            bricks: IndexMap::new(),
            z: IndexMap::new(),
        }
    }

    fn brick_at(&self, coord: Coord3D) -> Option<&BrickId> {
        let (x, y, z) = coord;
        self.z.get(&z).and_then(|z| z.get(&(x, y)))
    }

    fn add_brick(&mut self, id: BrickId, brick: Brick) -> bool {
        if self.bricks.contains_key(&id) {
            return false;
        }
        for coord in brick.coords() {
            if self.brick_at(coord).is_some() {
                return false;
            }
        }
        for coord in brick.coords() {
            let (x, y, z) = coord;
            self.z.entry(z).or_default().insert((x, y), id.clone());
        }
        self.bricks.insert(id, brick);
        true
    }

    pub fn remove_brick(&mut self, id: &BrickId) {
        let brick = self.bricks.swap_remove(id).unwrap();
        for coord in brick.coords() {
            let (x, y, z) = coord;
            let xy = self.z.get_mut(&z).unwrap();
            let removed = xy.swap_remove(&(x, y)).unwrap();
            assert_eq!(&removed, id);
            if xy.is_empty() {
                self.z.swap_remove(&z);
            }
        }
    }

    pub fn replace_brick(&mut self, id: &BrickId, brick: Brick) {
        self.remove_brick(id);
        self.add_brick(id.clone(), brick);
    }

    #[allow(dead_code)]
    pub fn x_view(&self) -> LateralView<'_> {
        LateralView {
            grid: self,
            x_side: true,
        }
    }
    #[allow(dead_code)]
    pub fn y_view(&self) -> LateralView<'_> {
        LateralView {
            grid: self,
            x_side: false,
        }
    }

    pub fn x_range(&self) -> RangeInclusive<usize> {
        let (&min, &max) = self
            .z
            .values()
            .flat_map(|xy| xy.keys())
            .map(|(x, _)| x)
            .minmax()
            .into_option()
            .unwrap();
        min..=max
    }
    pub fn y_range(&self) -> RangeInclusive<usize> {
        let (&min, &max) = self
            .z
            .values()
            .flat_map(|xy| xy.keys())
            .map(|(_, y)| y)
            .minmax()
            .into_option()
            .unwrap();
        min..=max
    }

    pub fn z_range(&self) -> RangeInclusive<usize> {
        let (&min, &max) = self.z.keys().minmax().into_option().unwrap();
        min..=max
    }

    pub fn supporters(&self, id: &BrickId) -> IndexSet<&BrickId> {
        let brick = self.bricks.get(id).unwrap();
        let brick_min_z = brick.min_z();
        let mut supporting = IndexSet::new();
        for (x, y) in brick.xys_at_z(brick_min_z) {
            if let Some(id_below) = self.brick_at((x, y, brick_min_z - 1)) {
                supporting.insert(id_below);
            }
        }
        supporting
    }
}

struct LateralView<'a> {
    grid: &'a BrickGrid,
    x_side: bool,
}

impl Display for LateralView<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn x_coord((a, other): (usize, usize), z: usize) -> (usize, usize, usize) {
            (a, other, z)
        }
        fn y_coord((a, other): (usize, usize), z: usize) -> (usize, usize, usize) {
            (other, a, z)
        }

        let grid = self.grid;
        let x_range = grid.x_range();
        let y_range = grid.y_range();
        let z_range = grid.z_range();

        let (range, other_range) = if self.x_side {
            (&x_range, &y_range)
        } else {
            (&y_range, &x_range)
        };
        for z in z_range.rev() {
            for a in range.clone() {
                let ids: IndexSet<_> = other_range
                    .clone()
                    .filter_map(|other| {
                        grid.brick_at(if self.x_side {
                            x_coord((a, other), z)
                        } else {
                            y_coord((a, other), z)
                        })
                    })
                    .collect();
                let c = match ids.iter().collect_vec().as_slice() {
                    [] => '.',
                    [id] => id.chars().next().unwrap(),
                    _ => '?',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f, " {}", z)?;
        }
        Ok(())
    }
}

impl From<Vec<Brick>> for BrickGrid {
    fn from(bricks: Vec<Brick>) -> Self {
        let mut grid = BrickGrid::new();
        for (i, brick) in bricks.iter().enumerate() {
            let brick_id = if bricks.len() <= 26 {
                ((b'A' + u8::try_from(i).unwrap()) as char).to_string()
            } else {
                i.to_string()
            };
            grid.add_brick(brick_id, brick.clone());
        }
        grid
    }
}
