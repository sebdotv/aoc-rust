use crate::challenge::Day;
use crate::utils::point3::Point3;
use anyhow::Result;
use disjoint::DisjointSet;
use itertools::Itertools;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (40, Some(129564)),
        part2_solutions: Some((25272, Some(42047840))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn parts(data: &str) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    let points = parse_input(data);
    let example = points.first().unwrap().x == 162;
    let mut pairs = (0..points.len())
        .tuple_combinations()
        .map(|(i, j)| (i, j, points[i].dist2(&points[j])))
        .collect_vec();
    pairs.sort_by_key(|(_, _, dist2)| *dist2);

    let p1_steps = if example { 10 } else { 1000 };
    let mut ds = DisjointSet::with_len(points.len());
    let mut steps = 0;
    for (i, j, _) in pairs {
        ds.join(i, j);
        steps += 1;
        let sets = ds.sets();
        if steps == p1_steps {
            let mut circuit_sizes = sets.iter().map(std::vec::Vec::len).collect_vec();
            circuit_sizes.sort_unstable();
            circuit_sizes.reverse();
            p1 = circuit_sizes.iter().take(3).product();
        }
        if sets.len() == 1 {
            p2 = points[i].x * points[j].x;
            break;
        }
    }

    (p1, p2)
}

fn part1(data: &str) -> Result<usize> {
    Ok(parts(data).0)
}
fn part2(data: &str) -> Result<usize> {
    Ok(parts(data).1)
}

type Scalar = usize;
type Point = Point3<Scalar>;

fn parse_input(data: &str) -> Vec<Point> {
    let mut points = vec![];
    for line in data.lines() {
        let (x, y, z) = line
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        points.push(Point { x, y, z });
    }
    points
}
