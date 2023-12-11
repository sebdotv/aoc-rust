use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Coord(pub usize, pub usize);

impl Coord {
    pub fn manhattan_distance(&self, other: &Coord) -> usize {
        let Coord(x1, y1) = self;
        let Coord(x2, y2) = other;
        x1.abs_diff(*x2) + y1.abs_diff(*y2)
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, EnumIter)]
pub enum Direction {
    N,
    S,
    E,
    W,
}
impl Direction {
    pub fn reverse(self) -> Direction {
        use Direction::*;
        match self {
            N => S,
            S => N,
            E => W,
            W => E,
        }
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    pub w: usize,
    pub h: usize,
    data: Vec<T>,
}

impl<T, E> Grid<T>
where
    E: Debug,
    T: FromStr<Err = E>,
{
    pub fn from_str(s: &str) -> Result<Self> {
        let lines = s.lines().collect_vec();
        Self::from_lines(&lines)
    }

    pub fn from_lines(lines: &[&str]) -> Result<Self> {
        let w = lines[0].len();
        let h = lines.len();
        let data = lines
            .iter()
            .flat_map(|line| {
                line.chars().map(|char| char.to_string()).map(|s| {
                    s.parse::<T>().map_err(|_| {
                        anyhow!("Could not parse {} as {}", s, std::any::type_name::<T>())
                    })
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { w, h, data })
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                write!(f, "{}", self.get(&Coord(x, y)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn from_data(data: Vec<Vec<T>>) -> Self {
        let h = data.len();
        let w = data[0].len();
        data.iter().for_each(|row| assert_eq!(row.len(), w));
        let data = data.into_iter().flatten().collect_vec();
        Self { w, h, data }
    }

    pub fn coords(&self) -> impl Iterator<Item = Coord> {
        (0..self.h)
            .cartesian_product(0..self.w)
            .map(|(y, x)| Coord(x, y))
    }

    pub fn row_coords(&self) -> Vec<Vec<Coord>> {
        (0..self.h)
            .map(|y| (0..self.w).map(|x| Coord(x, y)).collect_vec())
            .collect()
    }

    pub fn get(&self, coord: &Coord) -> &T {
        let Coord(x, y) = coord;
        self.data.get(x + y * self.w).unwrap()
    }

    pub fn neighbors(&self, coord: &Coord) -> Vec<Coord> {
        let Coord(x, y) = *coord;
        (-1..=1isize)
            .flat_map(|dx| {
                (-1..=1isize).filter_map(move |dy| {
                    if dx != 0 || dy != 0 {
                        let (x, y) = (
                            isize::try_from(x).unwrap() + dx,
                            isize::try_from(y).unwrap() + dy,
                        );
                        if x >= 0
                            && y >= 0
                            && x < isize::try_from(self.w).unwrap()
                            && y < isize::try_from(self.h).unwrap()
                        {
                            Some((usize::try_from(x).unwrap(), usize::try_from(y).unwrap()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            })
            .map(|(x, y)| Coord(x, y))
            .collect()
    }

    pub fn walk(&self, from: &Coord, dir: Direction) -> Option<Coord> {
        let Coord(x, y) = from;
        match dir {
            Direction::N if *y > 0 => Some(Coord(*x, y - 1)),
            Direction::S if *y < self.h - 1 => Some(Coord(*x, y + 1)),
            Direction::W if *x > 0 => Some(Coord(x - 1, *y)),
            Direction::E if *x < self.w - 1 => Some(Coord(x + 1, *y)),
            _ => None,
        }
    }

    // #[allow(dead_code)]
    pub fn transform<U, F>(&self, f: F) -> Grid<U>
    where
        F: Fn((&Coord, &T)) -> U,
    {
        Grid {
            w: self.w,
            h: self.h,
            data: self.coords().map(|c| f((&c, self.get(&c)))).collect_vec(),
        }
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            grid: self,
            curr: 0,
        }
    }
}

pub struct Iter<'a, T> {
    grid: &'a Grid<T>,
    curr: usize,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Copy,
{
    type Item = (Coord, T);

    fn next(&mut self) -> Option<Self::Item> {
        let current = (self.curr < self.grid.data.len())
            .then_some(Coord(self.curr % self.grid.w, self.curr / self.grid.w));
        self.curr += 1;
        current.map(|coord| (coord, *self.grid.get(&coord)))
    }
}
