use std::iter;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use itertools::Itertools;

use crate::challenge::Day;

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (95437, Some(1453349)),
        part2_solutions: Some((24933642, Some(2948823))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<usize> {
    let commands = parse_commands(data)?;
    let index = DirIndex::from(commands);

    let mut cache = IndexMap::new();
    let sum = index
        .directories()
        .map(|path| index.total_size(path, &mut cache))
        .filter(|&size| size <= 100000)
        .sum::<usize>();

    Ok(sum)
}

fn part2(data: &str) -> Result<usize> {
    let commands = parse_commands(data)?;
    let index = DirIndex::from(commands);

    let mut cache = IndexMap::new();
    let used = index.total_size(&vec![], &mut cache);
    let available = 70000000 - used;
    let required = 30000000;
    let must_free = required - available;

    let min = index
        .directories()
        .map(|path| index.total_size(path, &mut cache))
        .filter(|&size| size >= must_free)
        .min()
        .unwrap();

    Ok(min)
}

fn parse_commands(s: &str) -> Result<Vec<Command>> {
    s.lines()
        .collect_vec()
        .iter()
        .batching(|it| {
            it.next().map(|first| {
                iter::once(first)
                    .chain(it.peeking_take_while(|line| !line.starts_with("$ ")))
                    .join("\n")
            })
        })
        .map(|s| s.parse::<Command>())
        .collect::<Result<Vec<_>>>()
}

type Path = Vec<String>;

struct DirIndex {
    listings: IndexMap<Path, Vec<LsEntry>>,
}
impl DirIndex {
    fn directories(&self) -> impl Iterator<Item = &Path> {
        self.listings.keys()
    }
    fn total_size(&self, path: &Path, cache: &mut IndexMap<Path, usize>) -> usize {
        if let Some(cached) = cache.get(path) {
            return *cached;
        }
        let result = self
            .listings
            .get(path)
            .unwrap()
            .iter()
            .map(|entry| match entry {
                LsEntry::File(size, _) => *size,
                LsEntry::Dir(name) => self.total_size(&Self::make_path(path, name), cache),
            })
            .sum();
        cache.insert(path.clone(), result);
        result
    }
    fn make_path(parent: &Path, dir_name: &str) -> Path {
        let mut path = parent.clone();
        path.push(dir_name.to_owned());
        path
    }
}
impl From<Vec<Command>> for DirIndex {
    fn from(commands: Vec<Command>) -> Self {
        let mut current_path: Path = vec![];
        let mut listings: IndexMap<Path, Vec<LsEntry>> = IndexMap::new();
        for cmd in commands {
            match cmd {
                Command::Cd(cd_cmd) => match cd_cmd {
                    CdCommand::Slash => {
                        current_path.clear();
                    }
                    CdCommand::Up => {
                        current_path.pop().unwrap();
                    }
                    CdCommand::Dir(dir_name) => {
                        current_path.push(dir_name);
                    }
                },
                Command::Ls(ls_cmd) => {
                    let path = current_path.clone();
                    let prev = listings.insert(path, ls_cmd.entries);
                    assert!(prev.is_none());
                }
            }
        }
        Self { listings }
    }
}

#[derive(Debug)]
enum Command {
    Cd(CdCommand),
    Ls(LsCommand),
}
impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        s.parse::<LsCommand>()
            .map(Self::Ls)
            .or_else(|_| s.parse::<CdCommand>().map(Self::Cd))
            .map_err(|_| anyhow!("invalid command"))
    }
}

#[derive(Debug)]
enum CdCommand {
    Slash,
    Up,
    Dir(String),
}
impl FromStr for CdCommand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let dest = s
            .strip_prefix("$ cd ")
            .ok_or(anyhow!("strip prefix error"))?;
        match dest {
            "/" => Ok(Self::Slash),
            ".." => Ok(Self::Up),
            _ => Ok(Self::Dir(dest.to_owned())),
        }
    }
}

#[derive(Debug)]
struct LsCommand {
    entries: Vec<LsEntry>,
}
impl FromStr for LsCommand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines = s.lines();
        if lines.next().unwrap() != "$ ls" {
            return Err(anyhow!("invalid ls command"));
        }
        let entries = lines.map(str::parse).collect::<Result<_>>()?;
        Ok(Self { entries })
    }
}

#[derive(Debug)]
enum LsEntry {
    Dir(String),
    File(usize, #[allow(dead_code)] String),
}
impl FromStr for LsEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (prefix, name) = s.split_once(' ').ok_or(anyhow!("split error"))?;
        if prefix == "dir" {
            Ok(Self::Dir(name.to_owned()))
        } else {
            Ok(Self::File(prefix.parse()?, name.to_owned()))
        }
    }
}
