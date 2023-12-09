use std::{collections::HashMap, str::FromStr, sync::Arc, thread::{self, JoinHandle}};

use crate::{read_input, Solutions};
use num::integer::lcm;

#[derive(Debug)]
struct Node {
    l: [u8; 3],
    r: [u8; 3],
}

impl FromStr for Node {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            l: s.as_bytes()[1..4].try_into().unwrap(),
            r: s.as_bytes()[6..9].try_into().unwrap(),
        })
    }
}

fn part1(instructions: &[u8], map: &HashMap<[u8; 3], Node>) -> usize {
    let mut pos = [b'A'; 3];
    instructions
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(step, ins)| {
            if pos == [b'Z'; 3] {
                Some(step)
            } else if ins == &b'L' {
                pos = map.get(&pos).unwrap().l;
                None
            } else {
                pos = map.get(&pos).unwrap().r;
                None
            }
        })
        .unwrap()
}

// I was going to get rid of this but it was a convenient way to capture the starting position
#[derive(Debug)]
struct Ghost {
    pos: [u8; 3],
}

fn part2(instructions: Arc<[u8]>, map: Arc<HashMap<[u8; 3], Node>>) -> usize {
    let ghosts: Vec<Ghost> = map
        .keys()
        .filter_map(|key| {
            if key[2] == b'A' {
                Some(Ghost {
                    pos: *key,
                })
            } else {
                None
            }
        })
        .collect();

    println!("Starting {} ghosts", ghosts.len());
    let handles: Vec<JoinHandle<usize>> = ghosts
        .into_iter()
        .map(|mut ghost| {
            let cloned_instructions = instructions.clone();
            let cloned_map = map.clone();
            thread::spawn(move || {
                cloned_instructions.iter()
                    .cycle()
                    .enumerate()
                    .find_map(|(step, ins)| {
                        if ghost.pos[2] == b'Z' {
                            Some(step)
                        } else {
                            if ins == &b'L' {
                                ghost.pos = cloned_map.get(&ghost.pos).unwrap().l;
                            } else {
                                ghost.pos = cloned_map.get(&ghost.pos).unwrap().r;
                            }
                            None
                        }
                    })
                    .unwrap()
            })
        })
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .reduce(lcm)
        .unwrap()
}

pub fn solve() -> Solutions {
    let mut lines = read_input("08").into_iter();
    let instructions: Arc<[u8]> = Arc::from(lines.next().unwrap().trim().as_bytes());
    let map: HashMap<[u8; 3], Node> = lines
        .skip(1)
        .map(|line| {
            (
                line.as_bytes()[..3].try_into().unwrap(),
                line[6..].parse().unwrap(),
            )
        })
        .collect();
    let solution1 = part1(&instructions, &map);
    let solution2 = part2(instructions, Arc::new(map));

    Solutions(solution1.to_string(), solution2.to_string())
}
