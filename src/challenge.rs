use crate::input::read_data_file;
use anyhow::{anyhow, Result};
use std::ffi::OsStr;
use strum_macros::EnumIter;

pub type PartSolutions<T> = (T, Option<T>);
pub type PartSolver<T> = fn(&str) -> Result<T>;

#[derive(Debug)]
pub struct ChallengeDay<T> {
    pub part1_solutions: PartSolutions<T>,
    pub part2_solutions: Option<PartSolutions<T>>,
    pub part1_solver: PartSolver<T>,
    pub part2_solver: PartSolver<T>,
    pub source_file: &'static str,
}

impl<T> ChallengeDay<T> {
    pub fn solutions(&self, part: &Part) -> Option<&PartSolutions<T>> {
        match part {
            Part::Part1 => Some(&self.part1_solutions),
            Part::Part2 => self.part2_solutions.as_ref(),
        }
    }

    pub fn get_solver(&self, part: &Part) -> PartSolver<T> {
        match part {
            Part::Part1 => self.part1_solver,
            Part::Part2 => self.part2_solver,
        }
    }

    pub fn label(&self, part: &Part) -> String {
        let loc = self.source_file_location().unwrap();
        format!("{}::{}::{}", loc.dir, loc.stem, part)
    }

    fn source_file_location(&self) -> Result<SourceFileLocation> {
        let to_str = |s: &OsStr| {
            s.to_str()
                .ok_or(anyhow!("Could not convert OsStr to string"))
                .map(|s| s.to_owned())
        };
        let path = std::path::Path::new(&self.source_file);
        let dir = path
            .parent()
            .ok_or(anyhow!("Could not get parent directory"))?
            .file_name()
            .ok_or(anyhow!("Could not get file name of parent directory"))?;
        let stem = path.file_stem().ok_or(anyhow!("Could not get file stem"))?;
        Ok(SourceFileLocation {
            dir: to_str(dir)?,
            stem: to_str(stem)?,
        })
    }

    pub fn read_data_file(&self, data_file_name: &str) -> Result<String> {
        let loc = self.source_file_location()?;
        read_data_file(&loc.dir, &loc.stem, data_file_name)
    }
}

pub struct SourceFileLocation {
    pub dir: String,
    pub stem: String,
}

#[derive(Debug, EnumIter, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
pub enum ChallengeDayType {
    I32(ChallengeDay<i32>),
    U32(ChallengeDay<u32>),
    String(ChallengeDay<String>),
}

impl From<ChallengeDay<i32>> for ChallengeDayType {
    fn from(day: ChallengeDay<i32>) -> Self {
        ChallengeDayType::I32(day)
    }
}
impl From<ChallengeDay<u32>> for ChallengeDayType {
    fn from(day: ChallengeDay<u32>) -> Self {
        ChallengeDayType::U32(day)
    }
}
impl From<ChallengeDay<String>> for ChallengeDayType {
    fn from(day: ChallengeDay<String>) -> Self {
        ChallengeDayType::String(day)
    }
}
