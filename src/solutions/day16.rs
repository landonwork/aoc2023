use std::{
    cmp::{min, max},
    collections::{HashMap, HashSet},
};

use crate::Day;

pub struct Day16;

impl Day for Day16 {
    async fn part1(input: String) -> String {
        part1(&input).to_string()
    }

    async fn part2(input: String) -> String {
        part2(&input).to_string()
    }
}

fn part1(input: &str) -> usize {
    let mut graph = Graph::new(input);
    graph.add_emitter(1, 0, Direction::Right);
    graph.n_energized()
}

fn part2(input: &str) -> usize {
    let graph = Graph::new(input);

    let left_side =   (1..=graph.n_rows).map(|n| (n,              0,              Direction::Right));
    let right_side =  (1..=graph.n_rows).map(|n| (n,              graph.n_cols+1, Direction::Left));
    let top_side =    (1..=graph.n_cols).map(|n| (0,              n,              Direction::Down));
    let bottom_side = (1..=graph.n_cols).map(|n| (graph.n_rows+1, n,              Direction::Up));

    left_side.chain(right_side).chain(top_side).chain(bottom_side)
        .map(|(row, col, dir)| {
            let mut graph = graph.clone();
            graph.add_emitter(row, col, dir);
            graph.n_energized()
        })
        .max().unwrap_or(0)
}

#[derive(Clone)]
struct Graph {
    nodes: HashMap<(usize, usize), Glass>,
    edges: HashSet<Edge>,
    n_rows: usize,
    n_cols: usize,
    n_emitters: usize,
}

impl Graph {
    fn new(input: &str) -> Self {
        let mut n_rows = 0;
        let mut n_cols = 0;
        let nodes: HashMap<_, _> = input.trim().lines().enumerate().flat_map(|(row, line)| {
            n_cols = line.trim().len();
            n_rows += 1;
            line.trim().as_bytes().iter().enumerate().filter_map(move |(col, &b)| {
                let glass: Glass = b.into();
                (!matches!(glass, Glass::Empty)).then_some(((row + 1, col + 1), glass))
            })
        }).collect();

        Self { nodes, edges: HashSet::new(), n_rows, n_cols, n_emitters: 0 }
    }

    fn add_emitter(&mut self, row: usize, col: usize, dir: Direction) {
        self.nodes.insert((row, col), Glass::Emitter);
        self.n_emitters += 1;
        self.add_beams((row, col), dir);
    }

    fn out_of_bounds(&self, row: usize, col: usize) -> bool {
        row < 1 || col < 1 || row > self.n_rows || col > self.n_cols
    }

    fn add_beams(&mut self, start: (usize, usize), dir: Direction) {
        let mut coords = start;
        // Would love a do-while. Instead we have loop.
        loop {
            coords = {
                let new_coords = match dir {
                    Direction::Right => (coords.0, coords.1 + 1),
                    Direction::Left => (coords.0, coords.1 - 1),
                    Direction::Down => (coords.0 + 1, coords.1),
                    Direction::Up => (coords.0 - 1, coords.1),
                };
                // Are these coordinates out of bounds?
                // - Add an edge and terminate
                if self.out_of_bounds(new_coords.0, new_coords.1) {
                    self.edges.insert(ordered(start, coords));
                    break;
                }
                new_coords
            };

            // Do these coordinates match the coordinates of any nodes?
            // - Does the edge exist?
            // - - Yes => terminate
            // - - No  => add edge and recurse
            if let Some(glass) = self.nodes.get(&coords) {
                let edge = ordered(start, coords);
                if self.edges.contains(&edge) {
                    break;
                } else {
                    self.edges.insert(edge);
                    match (glass, dir) {
                        (Glass::ReflectForward, Direction::Up)
                            | (Glass::ReflectBackward, Direction::Down)
                            | (Glass::SplitHorizontal, Direction::Right) => {
                            self.add_beams(coords, Direction::Right);
                        }
                        (Glass::ReflectForward, Direction::Down)
                            | (Glass::ReflectBackward, Direction::Up)
                            | (Glass::SplitHorizontal, Direction::Left) => {
                            self.add_beams(coords, Direction::Left);
                        }
                        (Glass::ReflectForward, Direction::Left)
                            | (Glass::ReflectBackward, Direction::Right)
                            | (Glass::SplitVertical, Direction::Down) => {
                            self.add_beams(coords, Direction::Down);
                        }
                        (Glass::ReflectForward, Direction::Right)
                            | (Glass::ReflectBackward, Direction::Left)
                            | (Glass::SplitVertical, Direction::Up) => {
                            self.add_beams(coords, Direction::Up);
                        }
                        (Glass::SplitHorizontal, Direction::Up | Direction::Down) => {
                            self.add_beams(coords, Direction::Left);
                            self.add_beams(coords, Direction::Right);
                        }
                        (Glass::SplitVertical, Direction::Left | Direction::Right) => {
                            self.add_beams(coords, Direction::Up);
                            self.add_beams(coords, Direction::Down);
                        }
                        // unreachable-ish
                        (Glass::Empty | Glass::Emitter, _) => { break; }
                    }
                    break;
                }
            }
        }
    }

    fn n_energized(&self) -> usize {
        let energized: HashSet<_> = self.edges.iter()
            .flat_map(|(start, end)| {
                if start.0 == end.0 {
                    let (a, b) = ordered(start.1, end.1);
                    (a..=b).map(|x| (start.0, x)).collect::<Vec<_>>()
                } else {
                    let (a, b) = ordered(start.0, end.0);
                    (a..=b).map(|x| (x, start.1)).collect::<Vec<_>>()
                }
            })
            .collect();
        energized.len() - self.n_emitters
    }
}

fn ordered<T: Ord + Copy>(c1: T, c2: T) -> (T, T) {
    (min(c1, c2), max(c1, c2))
}

type Edge = ((usize, usize), (usize, usize));

#[allow(dead_code)] 
#[derive(Clone, Debug)]
enum Glass {
    SplitHorizontal,
    SplitVertical,
    ReflectForward, // forward slash '/'
    ReflectBackward, // back slash '\'
    Empty,
    Emitter,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// type Beam = (usize, usize, Direction);

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


#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 51);
    }

    #[test]
    fn test_part2_solvable() {
        let mut graph = Graph::new(TEST);
        graph.add_emitter(0, 4, Direction::Down);
        assert_eq!(graph.n_energized(), 51);
    }
}

