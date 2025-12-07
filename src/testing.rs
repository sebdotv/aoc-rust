use itertools::Itertools;

#[allow(dead_code)]
pub fn trim_lines(s: &str) -> String {
    s.trim().lines().map(str::trim).join("\n")
}
