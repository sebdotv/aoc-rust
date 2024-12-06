use crate::challenge::Day;
use crate::utils::grid::{Coord, Direction, Grid, Turn};
use anyhow::Result;
use indexmap::IndexSet;
use strum_macros::{EnumIter, EnumString};

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (41, Some(4890)),
        part2_solutions: Some((6, Some(1995))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

#[derive(EnumString, Debug, Eq, PartialEq, strum_macros::Display, Copy, Clone, EnumIter)]
enum Cell {
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "#")]
    Obstruction,
    #[strum(serialize = "O")]
    ObstructionOption,

    #[strum(serialize = "^")]
    GuardN,
    #[strum(serialize = "v")]
    GuardS,
    #[strum(serialize = "<")]
    GuardW,
    #[strum(serialize = ">")]
    GuardE,

    #[strum(serialize = "X")]
    Visited,
    #[strum(serialize = "|")]
    VisitedV,
    #[strum(serialize = "-")]
    VisitedH,
    #[strum(serialize = "+")]
    VisitedVH,
}
impl Cell {
    fn guard_direction(self) -> Option<Direction> {
        match self {
            Cell::GuardN => Some(Direction::N),
            Cell::GuardS => Some(Direction::S),
            Cell::GuardW => Some(Direction::W),
            Cell::GuardE => Some(Direction::E),
            _ => None,
        }
    }

    fn from_guard_direction(dir: Direction) -> Cell {
        match dir {
            Direction::N => Cell::GuardN,
            Direction::S => Cell::GuardS,
            Direction::W => Cell::GuardW,
            Direction::E => Cell::GuardE,
        }
    }
}

fn part1(data: &str) -> Result<usize> {
    let grid = part1_grid(data)?;
    let visited = grid
        .iter()
        .filter(|(_, cell)| cell == &Cell::Visited)
        .count();
    Ok(visited)
}

fn part1_grid(data: &str) -> Result<Grid<Cell>> {
    let mut grid: Grid<Cell> = data.parse()?;
    let (pos, dir) = remove_guard(&mut grid).unwrap();
    let (visited, _) = run_loop(&grid, pos, dir, true);
    visited.iter().for_each(|(coord, _)| {
        grid.set(coord, Cell::Visited);
    });
    Ok(grid)
}

fn remove_guard(grid: &mut Grid<Cell>) -> Option<(Coord, Direction)> {
    let (coord, dir) = find_guard(grid)?;
    grid.set(&coord, Cell::Empty);
    Some((coord, dir))
}
fn find_guard(grid: &Grid<Cell>) -> Option<(Coord, Direction)> {
    grid.iter()
        .find_map(|(coord, cell)| cell.guard_direction().map(|d| (coord, d)))
}

fn run_loop(
    grid: &Grid<Cell>,
    pos: Coord,
    dir: Direction,
    return_visited: bool,
) -> (IndexSet<(Coord, Direction)>, bool) {
    let mut pos = pos;
    let mut dir = dir;

    let mut visited = vec![vec![[false; 4]; grid.w]; grid.h];
    let mut looped = false;

    loop {
        let dir_u8: u8 = dir.into();
        let already_visited = visited[pos.1][pos.0][dir_u8 as usize];
        visited[pos.1][pos.0][dir as usize] = true;
        if already_visited {
            looped = true;
            break;
        }

        let proj_pos = grid.walk(&pos, dir);
        if let Some(proj_pos) = proj_pos {
            let mut next_pos = pos;
            let mut next_dir = dir;

            match grid.get(&proj_pos) {
                Cell::Empty => {
                    next_pos = proj_pos;
                }
                Cell::Obstruction | Cell::ObstructionOption => {
                    next_dir = Turn::Right.apply(dir);
                }
                Cell::GuardN
                | Cell::GuardS
                | Cell::GuardW
                | Cell::GuardE
                | Cell::Visited
                | Cell::VisitedV
                | Cell::VisitedH
                | Cell::VisitedVH => {
                    unreachable!()
                }
            }

            pos = next_pos;
            dir = next_dir;
        } else {
            break;
        }
    }

    let visited = if return_visited {
        let mut set = IndexSet::new();
        for (y, row) in visited.iter().enumerate() {
            for (x, dirs) in row.iter().enumerate() {
                let coord = Coord(x, y);
                for (dir, visited) in dirs.iter().enumerate() {
                    if *visited {
                        let dir_u8 = u8::try_from(dir).unwrap();
                        let dir = Direction::try_from(dir_u8).unwrap();
                        set.insert((coord, dir));
                    }
                }
            }
        }
        set
    } else {
        IndexSet::new()
    };
    (visited, looped)
}

fn part2(data: &str) -> Result<usize> {
    let (grid, pos, dir) = part2_prepare(data)?;

    // optimization: only consider normally visited cells
    let (visited, _) = run_loop(&grid, pos, dir, true);

    let visited_coords: IndexSet<_> = visited.into_iter().map(|(c, _)| c).collect();

    let options = visited_coords
        .into_iter()
        .filter(|c| {
            let (_, _, looped) = eval_option(c, &grid, &pos, dir, false);
            looped
        })
        .count();
    Ok(options)
}

fn part2_prepare(data: &str) -> Result<(Grid<Cell>, Coord, Direction)> {
    let mut grid: Grid<Cell> = data.parse()?;
    let (start_pos, start_dir) = remove_guard(&mut grid).unwrap();
    Ok((grid, start_pos, start_dir))
}

fn eval_option(
    c: &Coord,
    grid: &Grid<Cell>,
    start_pos: &Coord,
    start_dir: Direction,
    return_visited: bool,
) -> (Grid<Cell>, IndexSet<(Coord, Direction)>, bool) {
    let mut grid = grid.clone();
    grid.set(c, Cell::ObstructionOption);
    let (visited, looped) = run_loop(&grid, *start_pos, start_dir, return_visited);
    (grid, visited, looped)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::trim_lines;

    #[test]
    fn test_part1_example() {
        let example = &day().read_data_file("example").unwrap();
        let expected = "
            ....#.....
            ....XXXXX#
            ....X...X.
            ..#.X...X.
            ..XXXXX#X.
            ..X.X.X.X.
            .#XXXXXXX.
            .XXXXXXX#.
            #XXXXXXX..
            ......#X..
        ";
        assert_eq!(
            part1_grid(example).unwrap().to_string().trim(),
            trim_lines(expected)
        );
    }

    #[test]
    fn test_part2_example_options() -> Result<()> {
        let example = &day().read_data_file("example")?;
        let (grid, start_pos, start_dir) = part2_prepare(example)?;

        let test_option = |expected: String| {
            let c = expected
                .parse::<Grid<char>>()
                .unwrap()
                .iter()
                .find_map(|(coord, c)| (c == 'O').then_some(coord))
                .unwrap();
            let (grid, visited, looped) = eval_option(&c, &grid, &start_pos, start_dir, true);
            let str = part2_to_string(&grid, start_pos, start_dir, &visited)
                .trim()
                .to_owned();
            assert_eq!(str, expected);
            assert!(looped);
        };

        let expected = "
            ....#.....
            ....+---+#
            ....|...|.
            ..#.|...|.
            ....|..#|.
            ....|...|.
            .#.O^---+.
            ........#.
            #.........
            ......#...
        ";
        test_option(trim_lines(expected));

        let expected = "
            ....#.....
            ....+---+#
            ....|...|.
            ..#.|...|.
            ..+-+-+#|.
            ..|.|.|.|.
            .#+-^-+-+.
            ......O.#.
            #.........
            ......#...
        ";
        test_option(trim_lines(expected));

        let expected = "
            ....#.....
            ....+---+#
            ....|...|.
            ..#.|...|.
            ..+-+-+#|.
            ..|.|.|.|.
            .#+-^-+-+.
            .+----+O#.
            #+----+...
            ......#...
        ";
        test_option(trim_lines(expected));

        let expected = "
            ....#.....
            ....+---+#
            ....|...|.
            ..#.|...|.
            ..+-+-+#|.
            ..|.|.|.|.
            .#+-^-+-+.
            ..|...|.#.
            #O+---+...
            ......#...
        ";
        test_option(trim_lines(expected));

        let expected = "
            ....#.....
            ....+---+#
            ....|...|.
            ..#.|...|.
            ..+-+-+#|.
            ..|.|.|.|.
            .#+-^-+-+.
            ....|.|.#.
            #..O+-+...
            ......#...
        ";
        test_option(trim_lines(expected));

        let expected = "
            ....#.....
            ....+---+#
            ....|...|.
            ..#.|...|.
            ..+-+-+#|.
            ..|.|.|.|.
            .#+-^-+-+.
            .+----++#.
            #+----++..
            ......#O..
        ";
        test_option(trim_lines(expected));

        Ok(())
    }

    fn part2_to_string(
        grid: &Grid<Cell>,
        start_pos: Coord,
        start_dir: Direction,
        visited: &IndexSet<(Coord, Direction)>,
    ) -> String {
        let mut grid = grid.clone();

        let mut visited_vert: IndexSet<Coord> = IndexSet::new();
        let mut visited_horiz: IndexSet<Coord> = IndexSet::new();
        for (coord, dir) in visited {
            match dir {
                Direction::N | Direction::S => {
                    visited_vert.insert(*coord);
                }
                Direction::W | Direction::E => {
                    visited_horiz.insert(*coord);
                }
            }
        }

        for coord in visited_vert.union(&visited_horiz) {
            let v = visited_vert.contains(coord);
            let h = visited_horiz.contains(coord);
            if v && h {
                grid.set(coord, Cell::VisitedVH);
            } else if v {
                grid.set(coord, Cell::VisitedV);
            } else if h {
                grid.set(coord, Cell::VisitedH);
            } else {
                unreachable!();
            }
        }

        grid.set(&start_pos, Cell::from_guard_direction(start_dir));

        grid.to_string()
    }
}
