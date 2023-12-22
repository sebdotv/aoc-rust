use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::{bail, Result};
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;

use crate::challenge::Day;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (5, None),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let bricks = data
        .lines()
        .map(Brick::from_str)
        .collect::<Result<Vec<_>>>()?;
    // for brick in &bricks {
    //     println!("{:?}", brick);
    // }

    let mut grid = BrickGrid::new();
    for (i, brick) in bricks.iter().enumerate() {
        let brick_id = if bricks.len() <= 26 {
            ((b'A' + u8::try_from(i).unwrap()) as char).to_string()
        } else {
            i.to_string()
        };
        grid.add_brick(brick_id, brick.clone());
    }
    println!("x view:");
    println!("{}", grid.x_view());
    println!();
    println!("y view:");
    println!("{}", grid.y_view());
    println!();

    // let mut processed_bricks_by_z: IndexMap<usize, Brick> = IndexMap::new();

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
            println!("brick {}: min_z={}", id, z);
            if z == 1 {
                break;
            }

            let xys_at_z = brick.xys_at_z(z);
            println!("  xys_at_z={:?}", xys_at_z);

            let can_move = if let Some(x) = grid.z.get(&(z - 1)) {
                !xys_at_z.iter().any(|xy| x.contains_key(xy))
            } else {
                true
            };
            println!("  can_move={}", can_move);

            if !can_move {
                break;
            }

            println!("  moving down brick {}", id);

            grid.replace_brick(&id, brick.clone().move_down());
        }
    }

    println!("x view:");
    println!("{}", grid.x_view());
    println!();
    println!("y view:");
    println!("{}", grid.y_view());
    println!();

    Ok(0)
}

fn part2(_data: &str) -> Result<usize> {
    todo!()
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
        let brick = self.bricks.remove(id).unwrap();
        for coord in brick.coords() {
            let (x, y, z) = coord;
            let xy = self.z.get_mut(&z).unwrap();
            let removed = xy.remove(&(x, y)).unwrap();
            assert_eq!(&removed, id);
            if xy.is_empty() {
                self.z.remove(&z);
            }
        }
    }

    pub fn replace_brick(&mut self, id: &BrickId, brick: Brick) {
        self.remove_brick(id);
        self.add_brick(id.clone(), brick);
    }

    pub fn x_view(&self) -> LateralView<'_> {
        LateralView {
            grid: self,
            x_side: true,
        }
    }
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
}

struct LateralView<'a> {
    grid: &'a BrickGrid,
    x_side: bool,
}

impl<'a> Display for LateralView<'a> {
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
                    [&id] => id.chars().next().unwrap(),
                    _ => '?',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f, " {}", z)?;
        }
        Ok(())
    }
}
