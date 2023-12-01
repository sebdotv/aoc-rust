use crate::challenge::ChallengeDayType;
use itertools::Itertools;

pub mod challenge;
pub mod input;
#[allow(dead_code)]
#[allow(unreachable_code)]
#[allow(unused_variables)]
mod template;
pub mod year2022;
pub mod year2023;

pub fn all_challenge_days() -> Vec<ChallengeDayType> {
    vec![year2022::challenge_days(), year2023::challenge_days()]
        .into_iter()
        .flatten()
        .collect_vec()
}
