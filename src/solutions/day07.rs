use std::cmp::Ordering;

use axum::{response::Html, Form};

use crate::{PartInput, Solutions, read_input};

#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Type {
    High = 1,
    Pair = 2,
    TwoPair = 3,
    Three = 4,
    Full = 5,
    Four = 6,
    Five = 7,
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as u8).partial_cmp(&(*other as u8))
    }
}


impl Ord for Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}


#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as u8).partial_cmp(&(*other as u8))
    }
}


impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
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
            _ => unreachable!("{}", value as char),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    r#type: Type,
    cards: [Card; 5],
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let res @ Some(Ordering::Less) | res @ Some(Ordering::Greater) = self.r#type.partial_cmp(&other.r#type) {
            res
        } else {
            self.cards.partial_cmp(&other.cards)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let res @ Ordering::Less | res @ Ordering::Greater = self.r#type.cmp(&other.r#type) {
            res
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

fn hand_type(cards: &[u8]) -> Type {
    let mut counts = [(0u8, 0u8); 4];
    cards.iter().for_each(|b| {
        counts.iter_mut().find_map(|(key, val)| {
            if b == key || key == &0u8 {
                *key = *b;
                *val += 1;
                Some(())
            } else {
                None
            }
        });
    });
    counts
        .iter()
        .fold(Type::High, |r#type, (_, val)| match (r#type, val) {
            (_, 5) => Type::Five,
            (_, 4) => Type::Four,
            (Type::Three, 2) | (Type::Pair, 3) => Type::Full,
            (Type::Pair, 2) => Type::TwoPair,
            (_, 3) => Type::Three,
            (_, 2) => Type::Pair,
            (r#type, _) => r#type,
        })
}

pub fn part1(lines: &[String]) -> usize {
    let mut hands: Vec<_> = lines.iter()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            Hand {
                r#type: hand_type(cards.as_bytes()),
                cards: cards.as_bytes().iter().map(|&b| b.into()).collect::<Vec<Card>>().try_into().unwrap(),
                bid: bid.parse().unwrap()
            }
        })
        .collect();
    hands.sort();

    let cards: Vec<_> = hands.iter().filter_map(|hand| (hand.r#type == Type::High).then_some(&hand.cards)).collect();
    let mut sorted_cards = cards.clone();
    sorted_cards.sort();
    assert_eq!(cards, sorted_cards);

    hands
        .into_iter()
        .enumerate()
        .map(|(ind, hand)| -> usize {
            (ind + 1) * hand.bid
        })
        .sum()
}

pub fn solve() -> Solutions {
    let input = read_input("07");
    let solution1 = part1(&input);
    Solutions(solution1.to_string(), String::new())
}

pub async fn handle_part1(Form(PartInput { input }): Form<PartInput>) -> Html<String> {
    let input: Vec<_> = input.trim().split("\n").map(|x| x.to_owned()).collect();
    let ans = part1(&input);
    Html(ans.to_string())
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

    #[test]
    fn test_sort_card() {
        use Card::*;
        let test_hands = vec![
            [II, III, II, II, II],
            [II, IV, II, II, II],
            [II, IX, II, II, II],
            [II, X, II, II, II],
            [II, J, II, II, II],
            [II, A, II, II, II],
            [III, II, II, II, II],
            [IV, II, II, II, II],
            [IX, II, II, II, II],
            [X, II, II, II, II],
            [J, II, II, II, II],
            [A, II, II, II, II],
        ];

        let mut actual = vec![
            [II, IX, II, II, II],
            [II, X, II, II, II],
            [A, II, II, II, II],
            [IV, II, II, II, II],
            [III, II, II, II, II],
            [II, J, II, II, II],
            [II, A, II, II, II],
            [IX, II, II, II, II],
            [X, II, II, II, II],
            [J, II, II, II, II],
            [II, III, II, II, II],
            [II, IV, II, II, II],
        ];
        actual.sort();

        assert_eq!(actual, test_hands);
    }
}
