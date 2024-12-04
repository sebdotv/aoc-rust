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
    #[must_use]
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

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, EnumIter)]
pub enum DirectionDiag {
    NE,
    SE,
    SW,
    NW,
}
impl DirectionDiag {
    #[must_use]
    pub fn reverse(self) -> DirectionDiag {
        use DirectionDiag::*;
        match self {
            NE => SW,
            SW => NE,
            SE => NW,
            NW => SE,
        }
    }

    pub fn to_dirs(self) -> [Direction; 2] {
        use Direction::*;
        use DirectionDiag::*;
        match self {
            NE => [N, E],
            SE => [S, E],
            SW => [S, W],
            NW => [N, W],
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, EnumIter)]
pub enum Turn {
    Left,
    Right,
}
impl Turn {
    #[must_use]
    pub fn apply(self, dir: Direction) -> Direction {
        use Direction::*;
        use Turn::*;
        match (self, dir) {
            (Left, N) | (Right, S) => W,
            (Left, S) | (Right, N) => E,
            (Left, E) | (Right, W) => N,
            (Left, W) | (Right, E) => S,
        }
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    pub w: usize,
    pub h: usize,
    data: Vec<T>,
}

impl<T, E> FromStr for Grid<T>
where
    E: Debug,
    T: FromStr<Err = E>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        Self::from_lines(&lines)
    }
}

impl<T, E> Grid<T>
where
    E: Debug,
    T: FromStr<Err = E>,
{
    pub fn from_lines(lines: &[&str]) -> Result<Self> {
        let w = lines[0].len();
        let h = lines.len();
        let data = lines
            .iter()
            .flat_map(|line| {
                line.chars().map(|char| char.to_string()).map(|s| {
                    s.parse::<T>().map_err(|e| {
                        anyhow!(
                            "Could not parse {} as {}: {:?}",
                            s,
                            std::any::type_name::<T>(),
                            e
                        )
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
    /// # Panics
    ///
    /// Will panic if any row has a different length than the first row.
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
    pub fn col_coords(&self) -> Vec<Vec<Coord>> {
        (0..self.w)
            .map(|x| (0..self.h).map(|y| Coord(x, y)).collect_vec())
            .collect()
    }

    pub fn maybe_get(&self, coord: &Coord) -> Option<&T> {
        let Coord(x, y) = coord;
        if x >= &self.w || y >= &self.h {
            None
        } else if let Some(x) = self.data.get(x + y * self.w) {
            Some(x)
        } else {
            unreachable!()
        }
    }

    /// # Panics
    ///
    /// Will panic if out of bounds.
    pub fn get(&self, coord: &Coord) -> &T {
        self.maybe_get(coord).unwrap()
    }

    /// # Panics
    ///
    /// Will panic if out of bounds.
    pub fn set(&mut self, coord: &Coord, value: T) {
        let Coord(x, y) = coord;
        let v = self.data.get_mut(x + y * self.w).unwrap();
        *v = value;
    }

    /// # Panics
    ///
    /// Will panic if coordinates cannot be converted to usize/isize.
    pub fn neighbors_incl_diag(&self, coord: &Coord) -> Vec<Coord> {
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

    pub fn walk_diag(&self, from: &Coord, dir: DirectionDiag) -> Option<Coord> {
        self.walk_multiple(from, &dir.to_dirs())
    }

    pub fn walk_multiple(&self, from: &Coord, dirs: &[Direction]) -> Option<Coord> {
        dirs.iter()
            .try_fold(*from, |coord, dir| self.walk(&coord, *dir))
    }

    pub fn bottom_right(&self) -> Coord {
        Coord(self.w - 1, self.h - 1)
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

    /// # Panics
    ///
    /// Will panic if `rem_euclid` panics or if grid width/height cannot be converted to isize (unlikely).
    pub fn map_virtual(&self, x: isize, y: isize) -> Coord {
        let x = x.rem_euclid(isize::try_from(self.w).unwrap()) as usize;
        let y = y.rem_euclid(isize::try_from(self.h).unwrap()) as usize;
        Coord(x, y)
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn empty(w: usize, h: usize, value: T) -> Self {
        Self {
            w,
            h,
            data: vec![value; w * h],
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.into_iter()
    }
}

impl<'a, T: Copy> IntoIterator for &'a Grid<T> {
    type Item = (Coord, T);
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
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

impl<T> Iterator for Iter<'_, T>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_virtual_works() {
        let grid: Grid<u8> = Grid::from_lines(&["123", "456"]).unwrap();
        assert_eq!(grid.map_virtual(0, 0), Coord(0, 0));
        assert_eq!(grid.map_virtual(-1, 0), Coord(2, 0));
        assert_eq!(grid.map_virtual(0, -1), Coord(0, 1));
        assert_eq!(grid.map_virtual(3, 0), Coord(0, 0));
        assert_eq!(grid.map_virtual(0, 2), Coord(0, 0));
        assert_eq!(grid.map_virtual(-10, -10), Coord(2, 0));
    }
}
