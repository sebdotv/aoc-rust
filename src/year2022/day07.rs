use std::iter;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use itertools::Itertools;

use crate::challenge::Day;

pub fn day() -> Day<u32> {
    Day {
        part1_solutions: (95437, Some(1453349)),
        part2_solutions: None,
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<u32> {
    let commands = data
        .lines()
        .collect_vec()
        .iter()
        .batching(|it| {
            it.next().map(|first| {
                iter::once(first)
                    .chain(it.peeking_take_while(|line| !line.starts_with("$ ")))
                    .join("\n")
            })
        })
        .map(|s| s.parse::<Command>().unwrap())
        .inspect(|cmd| {
            println!("{:?}", cmd);
            println!();
        })
        .collect_vec();

    compute_dir_sizes(commands);

    Ok(0)
}

fn compute_dir_sizes(commands: Vec<Command>) -> Result<u32> {
    type Path = Vec<String>;
    struct DirIndex {
        dirs: IndexMap<Path, Dir>,
    }
    impl DirIndex {
        // fn insert(&mut self, dir: Dir) {
        //     let prev = self.dirs.insert(dir.path(), dir);
        //     assert!(prev.is_none());
        // }
        fn get(&self, path: &Path) -> &Dir {
            self.dirs.get(path).unwrap()
        }
        fn get_or_create(&mut self, parent_path: &Path, name: &String) -> &Dir {
            self.dirs
                .entry(Dir::make_path(parent_path, name))
                .or_insert_with(|| Dir {
                    name: name.clone(),
                    entries: vec![],
                    parent_path: parent_path.clone(),
                })
        }
    }
    struct Dir {
        name: String,
        entries: Vec<DirEntry>,
        parent_path: Path,
    }
    impl Dir {
        fn path(&self) -> Path {
            Self::make_path(&self.parent_path, &self.name)
        }
        fn make_path(parent_path: &Vec<String>, name: &String) -> Path {
            let mut path = parent_path.clone();
            path.push(name.clone());
            path
        }
    }
    enum DirEntry {
        Dir(Dir),
        File(u32, String),
    }
    let mut index = DirIndex {
        dirs: IndexMap::new(),
    };
    // let root = index.get_or_create(&vec![], &"".to_owned());
    // let mut current_dir = index.get(&Vec::new());

    let root: Path = vec![];
    let mut current_path: Path = root.clone();

    for cmd in commands {
        match cmd {
            Command::Cd(cd_cmd) => {
                current_path = match cd_cmd {
                    CdCommand::CdSlash => root.clone(),
                    CdCommand::CdUp => current_path.iter().dropping_back(1).cloned().collect(),
                    CdCommand::CdDir(dir_name) => current_path
                        .iter()
                        .chain(iter::once(&dir_name))
                        .cloned()
                        .collect(),
                };
                println!("{:?}", current_path);
            }
            Command::Ls(LsCommand { entries }) => todo!(),
        }
    }

    // let mut current_dir: Vec<&str> = vec![];
    // let mut dir_contents: IndexMap<Vec<&str>, Vec> = 0;
    Ok(0)
}

fn part2(_data: &str) -> Result<u32> {
    todo!()
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
    CdSlash,
    CdUp,
    CdDir(String),
}
impl FromStr for CdCommand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let dest = s
            .strip_prefix("$ cd ")
            .ok_or(anyhow!("strip prefix error"))?;
        match dest {
            "/" => Ok(Self::CdSlash),
            ".." => Ok(Self::CdUp),
            _ => Ok(Self::CdDir(dest.to_owned())),
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
        let entries = lines.map(|line| line.parse()).collect::<Result<_>>()?;
        Ok(Self { entries })
    }
}

#[derive(Debug)]
enum LsEntry {
    Dir(String),
    File(u32, String),
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