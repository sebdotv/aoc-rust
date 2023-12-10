use std::fmt::Debug;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Coord(pub usize, pub usize);

#[derive(Debug)]
pub struct Grid<T> {
    w: usize,
    h: usize,
    data: Vec<T>,
}
impl<T, E> Grid<T>
where
    E: Debug,
    T: FromStr<Err = E>,
{
    pub fn from_lines(lines: &[&str]) -> Self {
        let w = lines[0].len();
        let h = lines.len();
        let data = lines
            .iter()
            .flat_map(|line| {
                line.chars()
                    .map(|char| char.to_string())
                    .map(|s| s.parse::<T>().unwrap())
            })
            .collect::<Vec<_>>();
        Self { w, h, data }
    }
}

impl<T> Grid<T> {
    pub fn coords(&self) -> impl Iterator<Item = Coord> {
        (0..self.w)
            .cartesian_product(0..self.h)
            .map(|(x, y)| Coord(x, y))
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
}
