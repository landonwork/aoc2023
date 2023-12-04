use std::fs;

use crate::Solutions;

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>
}

#[derive(Debug)]
struct Number {
    row: i32,
    pos: i32,
    value: i32,
}

#[derive(Debug)]
struct Symbol {
    row: i32,
    pos: i32,
    symbol: u8
}

impl From<&[String]> for Schematic {
    fn from(lines: &[String]) -> Self {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();
        lines.iter()
            .enumerate()
            .for_each(|(row, line)| {
                let mut iter = line.as_bytes()
                    .iter()
                    .enumerate()
                    .peekable();
                while let Some((pos, b)) = iter.peek().copied() {
                    match *b {
                        mut b @ b'0'..=b'9' => {
                            let mut value = 0;
                            // It pains me that `while let` and `if let` cannot have guards
                            while matches!(b, b'0'..=b'9') {
                                value = value * 10 + (b - b'0') as i32;
                                iter.next();
                                if let Some((_, next)) = iter.peek() {
                                    b = **next;
                                } else {
                                    break;
                                }
                            }
                            numbers.push(Number { row: row as i32, pos: pos as i32, value })
                        }
                        b'.' => { iter.next(); }
                        symbol => {
                            symbols.push(Symbol { row: row as i32, pos: pos as i32, symbol });
                            iter.next();
                        }
                    }
                }
            });

        Self { numbers, symbols }
    }
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input/day3.txt")
        .unwrap()
        .replace("\r", "")
        .trim()
        .split("\n")
        .map(|x| x.to_owned())
        .collect()
}

fn part1(schematic: &Schematic) -> i32 {
    schematic.numbers
        .iter()
        .filter_map(|number| {

            schematic.symbols
                .iter()
                .any(|symbol| symbol.next_to(number))
                .then_some(number.value)
        })
        .sum()
}

impl Symbol {
    fn next_to(&self, number: &Number) -> bool {
        let length = number.value.checked_ilog10().unwrap_or(0) as i32;
        let pos_min = number.pos - 1;
        let pos_max = number.pos + 1 + length;
        let row_min = number.row - 1;
        let row_max = number.row + 1;
        self.pos >= pos_min && self.pos <= pos_max
            && self.row >= row_min && self.row <= row_max
    }
}

fn part2(schematic: &Schematic) -> i32 {
    schematic.symbols
        .iter()
        .filter_map(|symbol| {
            let gears: Option<[i32; 2]> = schematic.numbers
                .iter()
                .filter_map(|number| symbol.next_to(number).then_some(number.value))
                .collect::<Vec<i32>>()
                .try_into()
                .ok();
            gears
        })
        .map(|[a, b]| a * b)
        .sum()
}

pub fn solve() -> Solutions {
    let schematic = Into::into(read_input().as_slice());
    let solution1 = part1(&schematic);
    let solution2 = part2(&schematic);
    Solutions(solution1.to_string(), solution2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".replace("\r", "").trim().split("\n").map(|x| x.to_owned()).collect();
        assert_eq!(part1(&input.as_slice().into()), 4361);
    }

    #[test]
    fn test_length() {
        let length = 467i32.checked_ilog10().unwrap() as i32;
        assert_eq!(1 + length, 3);
    }
}
