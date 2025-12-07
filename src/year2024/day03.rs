use crate::challenge::Day;
use anyhow::Result;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::digit1;
use nom::combinator::{map, map_res, value};
use nom::multi::many1;
use nom::{Finish, IResult};
use nom_locate::{LocatedSpan, position};
use std::fmt::{Display, Formatter};

pub fn day() -> Day<usize> {
    Day {
        part1_solutions: (161, Some(173785482)),
        part2_solutions: Some((48, Some(83158140))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: true,
    }
}

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq)]
struct Mul<'a> {
    pub position: Span<'a>,
    pub a: usize,
    pub b: usize,
}
impl Display for Mul<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}*{}", self.a, self.b)
    }
}

#[derive(Debug, PartialEq)]
struct Switch<'a> {
    pub position: Span<'a>,
    pub enable: bool,
}
impl Display for Switch<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.enable { "do" } else { "don't" })
    }
}

#[derive(Debug, PartialEq)]
enum Instruction<'a> {
    Mul(Mul<'a>),
    Switch(Switch<'a>),
}
impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mul(mul) => write!(f, "{}", mul),
            Instruction::Switch(enabler) => write!(f, "{}", enabler),
        }
    }
}

fn parse_mul(s: Span) -> IResult<Span, Mul> {
    let mut number = map_res(digit1, |s: Span| s.fragment().parse::<usize>());
    let (s, position) = position(s)?;
    let (s, _) = tag("mul(")(s)?;
    let (s, a) = number.parse(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, b) = number.parse(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, Mul { position, a, b }))
}

fn skip1<T>(s: Span) -> IResult<Span, Option<T>> {
    let (s, _) = take(1usize)(s)?;
    Ok((s, None))
}

fn parse_muls(s: Span) -> IResult<Span, Vec<Mul>> {
    let mul_or_skip = alt((map(parse_mul, Some), skip1));
    let (s, muls) = many1(mul_or_skip).parse(s)?;
    Ok((s, muls.into_iter().flatten().collect()))
}

fn part1(data: &str) -> Result<usize> {
    let span = Span::new(data);
    let (s, muls) = parse_muls(span)
        .finish()
        .map_err(|e| anyhow::anyhow!("failed to parse: {}", e))?;
    assert!(s.fragment().is_empty());
    Ok(muls.iter().map(|mul| mul.a * mul.b).sum())
}

fn parse_switch(s: Span) -> IResult<Span, Switch> {
    let (s, position) = position(s)?;
    let (s, enable) = alt((value(true, tag("do()")), value(false, tag("don't()")))).parse(s)?;
    Ok((s, Switch { position, enable }))
}

fn parse_instructions(s: Span) -> IResult<Span, Vec<Instruction>> {
    let instruction = alt((
        map(parse_mul, |x| Some(Instruction::Mul(x))),
        map(parse_switch, |x| Some(Instruction::Switch(x))),
        skip1,
    ));
    let (s, instructions) = many1(instruction).parse(s)?;
    Ok((s, instructions.into_iter().flatten().collect()))
}

fn part2(data: &str) -> Result<usize> {
    let span = Span::new(data);
    let (s, instructions) = parse_instructions(span)
        .finish()
        .map_err(|e| anyhow::anyhow!("failed to parse: {}", e))?;
    assert!(s.fragment().is_empty());
    let mut enabled = true;
    let mut result = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Mul(mul) => {
                if enabled {
                    result += mul.a * mul.b;
                }
            }
            Instruction::Switch(switch) => {
                enabled = switch.enable;
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_parse_mul() {
        let data = "mul(2,3)";
        let (s, mul) = parse_mul(Span::new(data)).unwrap();
        assert!(s.fragment().is_empty());
        assert_eq!(mul.a, 2);
        assert_eq!(mul.b, 3);
        assert_eq!(mul.position.location_offset(), 0);
    }

    #[test]
    fn test_parse_muls() {
        let data = "amul(2,3)bmul(4,5)c";
        let (s, muls) = parse_muls(Span::new(data)).unwrap();
        assert!(s.fragment().is_empty());
        assert_eq!(muls.len(), 2);

        let mul = &muls[0];
        assert_eq!(mul.a, 2);
        assert_eq!(mul.b, 3);
        assert_eq!(mul.position.location_offset(), 1);

        let mul = &muls[1];
        assert_eq!(mul.a, 4);
        assert_eq!(mul.b, 5);
        assert_eq!(mul.position.location_offset(), 10);
    }

    #[test]
    fn test_parse_instructions() {
        let example2 = &day().read_data_file("example2").unwrap();
        let (s, instructions) = parse_instructions(Span::new(example2)).unwrap();
        assert!(s.fragment().is_empty());
        assert_eq!(instructions.len(), 6);
        assert_eq!(
            instructions.iter().map(ToString::to_string).join(" "),
            "2*4 don't 5*5 11*8 do 8*5"
        );
    }
}
