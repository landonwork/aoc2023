use std::str::FromStr;

use crate::{lines, Day};

pub struct Day04;

impl Day for Day04 {
    async fn part1(input: String) -> String {
        let cards: Vec<_> = lines(&input).iter().map(|line| line.parse().unwrap()).collect();
        part1(&cards).to_string()
    }

    async fn part2(input: String) -> String {
        let cards: Vec<_> = lines(&input).iter().map(|line| line.parse().unwrap()).collect();
        part2(cards).to_string()
    }
}


type Winning = Vec<[u8; 3]>;
type Given = Vec<[u8; 3]>;

#[derive(Debug)]
struct Card {
    winning: Winning,
    given: Given,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (winning, given) = s.split_once(':').unwrap().1.split_once(" |").unwrap();

        let winning = winning
            .as_bytes()
            .chunks_exact(3)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<[u8; 3]>>();
        let given = given
            .as_bytes()
            .chunks_exact(3)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<[u8; 3]>>();

        Ok(Card { winning, given })
    }
}

impl Card {
    fn match_count(&self) -> i64 {
        self.given
            .iter()
            .filter(|num| self.winning.contains(num))
            .count() as i64
    }

    fn score(&self) -> i64 {
        let num = self.match_count();
        if num == 0 {
            0
        } else {
            1 << (num - 1)
        }
    }
}

fn part1(cards: &[Card]) -> i64 {
    cards.iter().map(|card| card.score()).sum()
}

fn part2(cards: Vec<Card>) -> i64 {
    let mut cards = cards.into_iter().map(|card| (card, 1)).collect::<Vec<_>>();

    for i in 0..cards.len() {
        let (card, copies) = &cards[i];
        let num = card.match_count();
        let copies = *copies;

        for j in 0..num {
            if let Some((_, n)) = cards.get_mut(i + j as usize + 1) {
                *n += copies;
            }
        }
    }

    cards.into_iter().map(|(_, num)| num).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let cards = input
            .trim()
            .split('\n')
            .map(|line| line.parse().unwrap())
            .collect();
        assert_eq!(30, part2(cards));
    }
}
