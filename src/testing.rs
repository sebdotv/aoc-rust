use itertools::Itertools;

pub fn trim_lines(s: &str) -> String {
    s.trim().lines().map(str::trim).join("\n")
}
