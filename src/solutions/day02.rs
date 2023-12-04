use std::{fs, str::FromStr};

use crate::Solutions;

struct Game {
    number: i32,
    sets: Vec<Subset>
}

#[derive(Default)]
struct Subset {
    red: i32,
    blue: i32,
    green: i32,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, sets) = s.split_once(": ").unwrap();
        let number: i32 = game.split_at(5).1.parse().unwrap();
        let sets = sets.split("; ")
            .map(|set| set.parse().unwrap())
            .collect();
        Ok(Self { number, sets })
    }
}

impl FromStr for Subset {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut slf = Self::default();
        s.split(", ").for_each(|color| match color.split_once(" ").unwrap() {
            (num, "green") => { slf.green = num.parse().unwrap(); },
            (num, "blue") => { slf.blue = num.parse().unwrap(); },
            (num, "red") => { slf.red = num.parse().unwrap(); },
            _ => unreachable!()
        });
        Ok(slf)
    }
}

fn parse_input() -> Vec<String> {
    fs::read_to_string("input/day2.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|x| x.to_owned())
        .collect()
}

fn part1(input: &[String]) -> i32 {
    input
        .iter()
        .map(|line| line.parse().unwrap())
        .filter_map(|Game { number, sets }| {
            let max = sets.into_iter().reduce(|set1, set2| {
                Subset {
                    blue: std::cmp::max(set1.blue, set2.blue),
                    green: std::cmp::max(set1.green, set2.green),
                    red: std::cmp::max(set1.red, set2.red)
                }
            }).unwrap();
            (max.red <= 12 && max.green <= 13 && max.blue <= 14).then_some(number)
        })
        .sum()
}

fn part2(input: &[String]) -> i32 {
    input
        .iter()
        .map(|line| line.parse().unwrap())
        .map(|Game { number: _, sets }| {
            let min = sets.into_iter().reduce(|set1, set2| {
                Subset {
                    blue: std::cmp::max(set1.blue, set2.blue),
                    green: std::cmp::max(set1.green, set2.green),
                    red: std::cmp::max(set1.red, set2.red)
                }
            }).unwrap();
            min.red * min.green * min.blue
        })
        .sum()
}

pub fn solve() -> Solutions {
    let input = parse_input();
    let solution1 = part1(&input);
    let solution2 = part2(&input);
    Solutions(solution1.to_string(), solution2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let _: Vec<Game> = parse_input().iter().map(|line| line.parse().unwrap()).collect();
    }
}
