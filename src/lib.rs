use crate::challenge::DayWrapper;
use itertools::Itertools;

pub mod challenge;
pub mod input;
#[allow(dead_code)]
#[allow(unreachable_code)]
#[allow(unused_variables)]
mod template;
#[cfg(test)]
mod testing;
pub mod utils;
#[cfg(feature = "previous-years")]
pub mod year2022;
#[cfg(feature = "previous-years")]
pub mod year2023;
#[cfg(feature = "previous-years")]
pub mod year2024;
pub mod year2025;

#[must_use]
pub fn all_challenge_days() -> Vec<DayWrapper> {
    vec![
        #[cfg(feature = "previous-years")]
        year2022::challenge_days(),
        #[cfg(feature = "previous-years")]
        year2023::challenge_days(),
        #[cfg(feature = "previous-years")]
        year2024::challenge_days(),
        year2025::challenge_days(),
    ]
    .into_iter()
    .flatten()
    .collect_vec()
}
