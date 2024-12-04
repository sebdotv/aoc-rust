use std::fmt::Debug;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use strum_macros::EnumString;

use crate::challenge::Day;

pub fn day() -> Day<u32> {
    Day {
        part1_solutions: (6440, Some(252656917)),
        part2_solutions: Some((5905, Some(253499763))),
        part1_solver: part1,
        part2_solver: part2,
        source_file: file!(),
        distinct_examples: false,
    }
}

fn part1(data: &str) -> Result<u32> {
    let hands = parse_hands(data)?;
    let hands = hands
        .into_iter()
        .map(|hand| {
            let hand_type = hand_type(&hand.cards);
            (hand, hand_type)
        })
        .collect_vec();
    let sum = total_score(&hands, |card| card as usize);
    Ok(sum)
}

fn part2(data: &str) -> Result<u32> {
    let hands = parse_hands(data)?;
    let hands = hands
        .into_iter()
        .map(|hand| {
            use HandType::*;
            let (jokers, rest) = hand
                .cards
                .iter()
                .partition::<Vec<Card>, _>(|&&card| card == Card::_J);
            let jokers = jokers.len();
            let hand_type = if jokers == 5 {
                FiveOfAKind // special case for 5 jokers
            } else {
                let hand_type = hand_type(&rest); // eval hand without jokers
                let improved_hand_type = match hand_type {
                    FiveOfAKind | FullHouse => None,
                    FourOfAKind => (jokers > 0).then_some(FiveOfAKind),
                    ThreeOfAKind => match jokers {
                        0 => None,
                        1 => Some(FourOfAKind),
                        2 => Some(FiveOfAKind),
                        _ => unreachable!("Invalid hand: {:?}", hand),
                    },
                    TwoPair => match jokers {
                        0 => None,
                        1 => Some(FullHouse),
                        _ => unreachable!("Invalid hand: {:?}", hand),
                    },
                    OnePair => match jokers {
                        0 => None,
                        1 => Some(ThreeOfAKind),
                        2 => Some(FourOfAKind),
                        3 => Some(FiveOfAKind),
                        _ => unreachable!("Invalid hand: {:?}", hand),
                    },
                    HighCard => match jokers {
                        0 => None,
                        1 => Some(OnePair),
                        2 => Some(ThreeOfAKind),
                        3 => Some(FourOfAKind),
                        4 => Some(FiveOfAKind),
                        _ => unreachable!("Invalid hand: {:?}", hand),
                    },
                };
                improved_hand_type.unwrap_or(hand_type) // apply joker improvements or keep original
            };
            (hand, hand_type)
        })
        .collect_vec();
    let sum = total_score(
        &hands,
        |card| if card == Card::_J { 100 } else { card as usize }, // jokers have lowest rank
    );
    Ok(sum)
}

fn parse_hands(data: &str) -> Result<Vec<Hand>> {
    data.lines()
        .map(str::parse::<Hand>)
        .collect::<Result<Vec<_>>>()
}

fn total_score(hands: &[(Hand, HandType)], card_rank: fn(Card) -> usize) -> u32 {
    let sum = hands
        .iter()
        .sorted_by_key(|(hand, hand_type)| (hand_type, hand.cards.map(card_rank)))
        .rev()
        .enumerate()
        .map(|(i, (hand, _))| {
            let rank = i + 1;
            hand.bid * u32::try_from(rank).unwrap()
        })
        .sum::<u32>();
    sum
}

type Cards = [Card; 5];

#[derive(Debug)]
struct Hand {
    cards: Cards,
    bid: u32,
}

fn hand_type(cards: &[Card]) -> HandType {
    let counts = cards.iter().counts();
    let groups = counts
        .iter()
        .filter(|(_, &count)| count > 1) // only keep pairs or more
        .sorted_by_key(|(_, &count)| count)
        .rev()
        .collect_vec();
    match groups.as_slice() {
        [(_, 5)] => HandType::FiveOfAKind,
        [(_, 4)] => HandType::FourOfAKind,
        [(_, 3), (_, 2)] => HandType::FullHouse,
        [(_, 3)] => HandType::ThreeOfAKind,
        [(_, 2), (_, 2)] => HandType::TwoPair,
        [(_, 2)] => HandType::OnePair,
        [] => HandType::HighCard,
        _ => unreachable!("Invalid hand: {:?}", cards),
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (cards, bid) = s
            .split_once(' ')
            .ok_or(anyhow!("Cannot split hand: {}", s))?;
        let cards = cards
            .chars()
            .map(|s| {
                s.to_string()
                    .parse::<Card>()
                    .with_context(|| format!("Invalid card: {}", s))
            })
            .collect::<Result<Vec<_>>>()?;
        let cards = cards
            .try_into()
            .map_err(|_e| anyhow!("Invalid number of cards: {}", s))?;
        let bid = bid.parse::<u32>()?;
        Ok(Self { cards, bid })
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, PartialEq, Hash, EnumString, strum_macros::Display, Copy, Clone)]
enum Card {
    #[strum(serialize = "A")]
    _A,
    #[strum(serialize = "K")]
    _K,
    #[strum(serialize = "Q")]
    _Q,
    #[strum(serialize = "J")]
    _J,
    #[strum(serialize = "T")]
    _T,
    #[strum(serialize = "9")]
    _9,
    #[strum(serialize = "8")]
    _8,
    #[strum(serialize = "7")]
    _7,
    #[strum(serialize = "6")]
    _6,
    #[strum(serialize = "5")]
    _5,
    #[strum(serialize = "4")]
    _4,
    #[strum(serialize = "3")]
    _3,
    #[strum(serialize = "2")]
    _2,
}
