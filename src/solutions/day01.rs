use std::fs;

use crate::Solutions;

fn read_input() -> Vec<String> {
    fs::read_to_string("input/day1.txt")
        .unwrap()
        .split("\n")
        .map(|x| x.to_owned())
        .collect()
}

fn part1(input: &[String]) -> i32 {
    input.iter()
        .map(|line| {
            let mut digits = None;
            line.as_bytes().iter().for_each(|b| match b {
                b'0'..=b'9' => match digits {
                    None => { digits = Some((b, b)); }
                    Some((_, ref mut last)) => { *last = b; }
                },
                _ => ()
            });
            if let Some((first, last)) = digits {
                to_i32(*first) * 10 + to_i32(*last)
            } else {
                0
            }
        })
        .sum()
}

fn to_i32(b: u8) -> i32 {
    (b - b'0') as i32
}

fn part2(input: &[String]) -> i32 {
    input.iter()
        .map(|line| parse_line(line, None))
        .map(|(first, last)| first * 10 + last)
        .sum()
}

fn parse_line(line: &str, digits: Option<(i32, i32)>) -> (i32, i32) {
    if line.is_empty() { digits.unwrap_or((0, 0)) } else {
        let next = line.split_at(1).1;
        if line.starts_with("0") || line.starts_with("zero") {
            parse_line(next, digits.map_or_else(|| Some((0, 0)), |(a, _)| Some((a, 0))))
        } else if line.starts_with("1") || line.starts_with("one") {
            parse_line(next, digits.map_or_else(|| Some((1, 1)), |(a, _)| Some((a, 1))))
        } else if line.starts_with("2") || line.starts_with("two") {
            parse_line(next, digits.map_or_else(|| Some((2, 2)), |(a, _)| Some((a, 2))))
        } else if line.starts_with("3") || line.starts_with("three") {
            parse_line(next, digits.map_or_else(|| Some((3, 3)), |(a, _)| Some((a, 3))))
        } else if line.starts_with("4") || line.starts_with("four") {
            parse_line(next, digits.map_or_else(|| Some((4, 4)), |(a, _)| Some((a, 4))))
        } else if line.starts_with("5") || line.starts_with("five") {
            parse_line(next, digits.map_or_else(|| Some((5, 5)), |(a, _)| Some((a, 5))))
        } else if line.starts_with("6") || line.starts_with("six") {
            parse_line(next, digits.map_or_else(|| Some((6, 6)), |(a, _)| Some((a, 6))))
        } else if line.starts_with("7") || line.starts_with("seven") {
            parse_line(next, digits.map_or_else(|| Some((7, 7)), |(a, _)| Some((a, 7))))
        } else if line.starts_with("8") || line.starts_with("eight") {
            parse_line(next, digits.map_or_else(|| Some((8, 8)), |(a, _)| Some((a, 8))))
        } else if line.starts_with("9") || line.starts_with("nine") {
            parse_line(next, digits.map_or_else(|| Some((9, 9)), |(a, _)| Some((a, 9))))
        } else {
            parse_line(next, digits)
        }
    }
}

pub fn solve() -> Solutions {
    let input = read_input();
    let solution1 = part1(&input);
    let solution2 = part2(&input);
    Solutions(solution1.to_string(), solution2.to_string())
}
