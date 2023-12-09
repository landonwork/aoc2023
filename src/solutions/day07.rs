use axum::{response::Html, Form};
use serde::{Serialize, Deserialize};

use crate::{Solutions, lines};

#[repr(u8)]
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone, Copy)]
enum Type {
    High = 1,
    Pair = 2,
    TwoPair = 3,
    Three = 4,
    Full = 5,
    Four = 6,
    Five = 7,
}

#[repr(u8)]
#[derive(Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
enum Card {
    II = 2,
    III = 3,
    IV = 4,
    V = 5,
    VI = 6,
    VII = 7,
    VIII = 8,
    IX = 9,
    X = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            b'2' => Card::II,
            b'3' => Card::III,
            b'4' => Card::IV,
            b'5' => Card::V,
            b'6' => Card::VI,
            b'7' => Card::VII,
            b'8' => Card::VIII,
            b'9' => Card::IX,
            b'T' => Card::X,
            b'J' => Card::J,
            b'Q' => Card::Q,
            b'K' => Card::K,
            b'A' => Card::A,
            _ => unreachable!()
        }
    }
}

#[derive(PartialEq, Eq)]
struct Hand<'a> {
    r#type: Type,
    cards: &'a [u8],
}

impl<'a> From<&'a str> for Hand<'a> {
    fn from(s: &'a str) -> Self {
        let bytes = s.as_bytes();
        Hand { r#type: hand_type(bytes), cards: bytes }
    }
}

fn hand_type(cards: &[u8]) -> Type {
    let mut counts = [(0u8, 0u8); 4];
    cards.iter()
        .for_each(|b| {
            counts.iter_mut()
                .find_map(|(key, val)| {
                    if b == key || key == &0u8 {
                        *key = *b;
                        *val += 1;
                        Some(())
                    } else {
                        None
                    }
                });
        });
    counts.iter().fold(Type::High, |r#type, (_, val)| match (r#type, val) {
        (_, 5) => Type::Five,
        (_, 4) => Type::Four,
        (Type::Three, 2) | (Type::Pair, 3) => Type::Full,
        (Type::Pair, 2) => Type::TwoPair,
        (_, 3) => Type::Three,
        (_, 2) => Type::Pair,
        (r#type, _) => r#type,
    })
}

#[derive(Serialize, Deserialize)]
pub struct Part1 { input: String }
pub async fn part1(Form(Part1 { input }): Form<Part1>) -> Html<String> {
    let lines = lines(&input);
    let mut indices = (1..=lines.len()).collect::<Vec<_>>();
    indices.sort_by_key(|i| {
        let byte_slice = lines[i-1][..5].as_bytes();
        (hand_type(byte_slice), byte_slice.iter().map(|&b| b.into()).collect::<Vec<Card>>())
    });
    let ans: usize = indices.into_iter()
        .zip(lines)
        .map(|(ind, line)| -> usize {
            let bid: usize = line.split_once(" ").unwrap().1.parse().unwrap();
            ind * bid
        })
        .sum();
    Html(format!("Solution: {ans}"))
}

pub fn solve() -> Solutions {
    Solutions(
        String::new(),
        String::new()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        let cases = [
            ("JJJJJ", Type::Five),
            ("JJJAJ", Type::Four),
            ("2T2T2", Type::Full),
            ("99329", Type::Three),
            ("19129", Type::TwoPair),
            ("18129", Type::Pair),
            ("38129", Type::High),
        ];
        for (hand, ans) in cases.into_iter() {
            assert_eq!(hand_type(hand.as_bytes()), ans);
        }
    }
}

