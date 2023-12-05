use std::ffi::OsStr;

use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use strum_macros::EnumIter;

use crate::input::read_data_file;

pub type PartSolutions<T> = (T, Option<T>);
pub type PartSolver<T> = fn(&str) -> Result<T>;

#[derive(Debug)]
pub struct Day<T> {
    pub part1_solutions: PartSolutions<T>,
    pub part2_solutions: Option<PartSolutions<T>>,
    pub part1_solver: PartSolver<T>,
    pub part2_solver: PartSolver<T>,
    pub source_file: &'static str,
    pub distinct_examples: bool,
}

impl<T> Day<T> {
    pub fn solutions(&self, part: Part) -> Option<&PartSolutions<T>> {
        match part {
            Part::Part1 => Some(&self.part1_solutions),
            Part::Part2 => self.part2_solutions.as_ref(),
        }
    }

    pub fn get_solver(&self, part: Part) -> PartSolver<T> {
        match part {
            Part::Part1 => self.part1_solver,
            Part::Part2 => self.part2_solver,
        }
    }

    pub fn label(&self, part: Part) -> Result<String> {
        let loc = self.source_file_location()?;
        Ok(format!("{}::{}::{}", loc.dir, loc.stem, part))
    }

    fn source_file_location(&self) -> Result<SourceFileLocation> {
        let to_str = |s: &OsStr| {
            s.to_str()
                .ok_or(anyhow!("Could not convert OsStr to string"))
                .map(ToOwned::to_owned)
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

impl SourceFileLocation {
    pub fn date(&self) -> Result<NaiveDate> {
        let year = self
            .dir
            .strip_prefix("year")
            .ok_or(anyhow!("Could not strip prefix"))?
            .parse::<i32>()?;
        let day = self
            .stem
            .strip_prefix("day")
            .ok_or(anyhow!("Could not strip prefix"))?
            .parse::<u32>()?;
        NaiveDate::from_ymd_opt(year, 12, day)
            .ok_or(anyhow!("Could not create date from components"))
    }
}

#[derive(Debug, EnumIter, strum_macros::Display, Copy, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Part {
    Part1 = 1,
    Part2 = 2,
}

#[derive(Debug)]
pub enum DayWrapper {
    I32(Day<i32>),
    U32(Day<u32>),
    U64(Day<u64>),
    String(Day<String>),
}

impl DayWrapper {
    pub fn source_file_location(&self) -> Result<SourceFileLocation> {
        use DayWrapper::*;
        match self {
            I32(day) => day.source_file_location(),
            U32(day) => day.source_file_location(),
            U64(day) => day.source_file_location(),
            String(day) => day.source_file_location(),
        }
    }
}

impl From<Day<i32>> for DayWrapper {
    fn from(day: Day<i32>) -> Self {
        DayWrapper::I32(day)
    }
}
impl From<Day<u32>> for DayWrapper {
    fn from(day: Day<u32>) -> Self {
        DayWrapper::U32(day)
    }
}
impl From<Day<u64>> for DayWrapper {
    fn from(day: Day<u64>) -> Self {
        DayWrapper::U64(day)
    }
}
impl From<Day<String>> for DayWrapper {
    fn from(day: Day<String>) -> Self {
        DayWrapper::String(day)
    }
}
