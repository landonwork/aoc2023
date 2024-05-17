use std::collections::{VecDeque, HashSet};

use crate::{lines, Day};

pub struct Day16;

impl Day for Day16 {
    async fn part1(input: String) -> String {
        part1(input).to_string()
    }
}

enum Glass {
    SplitHorizontal,
    SplitVertical,
    ReflectForward, // forward slash '/'
    ReflectBackward, // back slash '\'
    Empty
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Beam = (usize, usize, Direction);

impl From<u8> for Glass {
    fn from(value: u8) -> Self {
        match value {
            b'/' => Self::ReflectForward,
            b'\\' => Self::ReflectBackward,
            b'|' => Self::SplitVertical,
            b'-' => Self::SplitHorizontal,
            b'.' => Self::Empty,
            _ => unreachable!("{value}")
        }
    }
}

fn move_up((row, col, _dir): Beam, _height: usize, beams: &mut VecDeque<Beam>) {
    if row > 0 { beams.push_back((row - 1, col, Direction::Up)); }
}

fn move_down((row, col, _dir): Beam, height: usize, beams: &mut VecDeque<Beam>) {
    if row < height - 1 { beams.push_back((row + 1, col, Direction::Down)); }
}


fn move_left((row, col, _dir): Beam, _width: usize, beams: &mut VecDeque<Beam>) {
    if col > 0 { beams.push_back((row, col - 1, Direction::Left)); }
}

fn move_right((row, col, _dir): Beam, width: usize, beams: &mut VecDeque<Beam>) {
    if col < width - 1 { beams.push_back((row, col + 1, Direction::Right)); }
}

fn part1(input: String) -> usize {
    let map: Vec<Vec<Glass>> = lines(&input).iter().map(|line| line.as_bytes().iter().map(|&b| b.into()).collect()).collect();
    let mut beams = VecDeque::from([(0usize, 0usize, Direction::Right)]);
    let mut energized = HashSet::new();
    let height = map.len();
    let width = map[0].len();
    let mut count = 0;
    while let Some(beam) = beams.pop_front() {
        if count % 1000 == 0 {
            println!("{count}");
        }
        count += 1;
        energized.insert((beam.0, beam.1));
        match (beam.2, map.get(beam.0).and_then(|r| r.get(beam.1)).unwrap()) {
            // Nothing happens
            (Direction::Up, Glass::Empty | Glass::SplitVertical) => {
                move_up(beam, height, &mut beams);
            }
            (Direction::Down, Glass::Empty | Glass::SplitVertical) => {
                move_down(beam, height, &mut beams);
            }
            (Direction::Left, Glass::Empty | Glass::SplitHorizontal) => {
                move_left(beam, width, &mut beams);
            }
            (Direction::Right, Glass::Empty | Glass::SplitHorizontal) => {
                move_right(beam, width, &mut beams);
            }
            // splitting
            (Direction::Up | Direction::Down, Glass::SplitHorizontal) => {
                move_left(beam, width, &mut beams);
                move_right(beam, width, &mut beams);
            }
            (Direction::Left | Direction::Right, Glass::SplitVertical) => {
                move_up(beam, height, &mut beams);
                move_down(beam, height, &mut beams);
            }
            // reflections
            (Direction::Up, Glass::ReflectForward) | (Direction::Down, Glass::ReflectBackward) => { // '/'
                move_right(beam, height, &mut beams);
            }
            (Direction::Down, Glass::ReflectForward) | (Direction::Up, Glass::ReflectBackward) => { // '/'
                move_left(beam, height, &mut beams);
            }
            (Direction::Left, Glass::ReflectForward) | (Direction::Right, Glass::ReflectBackward) => { // '/'
                move_down(beam, width, &mut beams);
            }
            (Direction::Right, Glass::ReflectForward) | (Direction::Left, Glass::ReflectBackward) => { // '/'
                move_up(beam, width, &mut beams);
            }
        }
    }

    energized.len()
}
