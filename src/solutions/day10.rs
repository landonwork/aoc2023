// use std::{collections::HashSet, hash::Hash};

use crate::Day;

pub struct Day10;

impl Day for Day10 {}

// type Point = (usize, usize);
// 
// struct BiEdge(pub Point, pub Point);
// 
// impl Eq for BiEdge { }
// 
// impl PartialEq for BiEdge {
//     fn eq(&self, other: &Self) -> bool {
//         std::cmp::min(self.0, self.1) == std::cmp::min(other.0, other.1)
//             && std::cmp::max(self.0, self.1) == std::cmp::max(other.0, other.1)
//     }
// }
// 
// impl Hash for BiEdge {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         std::cmp::min(self.0, self.1).hash(state);
//         std::cmp::max(self.0, self.1).hash(state);
//     }
// }
// 
// impl From<(Point, Point)> for BiEdge {
//     fn from(value: (Point, Point)) -> Self {
//         Self(value.0, value.1)
//     }
// }
// 
// #[derive(Default)]
// struct Graph {
//     ids: HashSet<Point>,
//     uni_edges: HashSet<(Point, Point)>,
//     bi_edges: HashSet<BiEdge>,
// }
// 
// impl Graph {
//     fn add_uni_edge(&mut self, edge: (Point, Point)) {
//         self.uni_edges.insert(edge);
//         if self.uni_edges.contains(&(edge.1, edge.0)) {
//             self.bi_edges.insert(edge.into());
//         }
//     }
// }
// 
// fn part1(input: &[String]) -> i64 {
//     let mut graph = Graph::default();
//     let mut start = None;
//     input.iter()
//         .enumerate()
//         .for_each(|(row, line)| {
//             line.as_bytes()
//                 .iter()
//                 .enumerate()
//                 .for_each(|(col, b)| {
//                     let point = (row, col);
//                     match b {
//                         b'|' => {
//                             graph.ids.insert(point);
//                             graph.add_uni_edge((point, (point.0 - 1, point.1)));
//                             graph.add_uni_edge((point, (point.0 + 1, point.1)));
//                         },
//                         b'-' => {
//                             graph.ids.insert(point);
//                             graph.add_uni_edge((point, (point.0, point.1 - 1)));
//                             graph.add_uni_edge((point, (point.0, point.1 + 1)));
//                         },
//                         b'L' => {
//                             graph.ids.insert(point);
//                             graph.add_uni_edge((point, (point.0 - 1, point.1)));
//                             graph.add_uni_edge((point, (point.0, point.1 + 1)));
//                         },
//                         b'J' => {
//                             graph.ids.insert(point);
//                             graph.add_uni_edge((point, (point.0 - 1, point.1)));
//                             graph.add_uni_edge((point, (point.0, point.1 - 1)));
//                         },
//                         b'F' => {
//                             graph.ids.insert(point);
//                             graph.add_uni_edge((point, (point.0 + 1, point.1)));
//                             graph.add_uni_edge((point, (point.0, point.1 + 1)));
//                         },
//                         b'7' => {
//                             graph.ids.insert(point);
//                             graph.add_uni_edge((point, (point.0 + 1, point.1)));
//                             graph.add_uni_edge((point, (point.0, point.1 - 1)));
//                         },
//                         b'S' => {
//                             start = Some(point);
//                             graph.ids.insert((row, col));
//                             graph.add_uni_edge((point, (point.0 + 1, point.1)));
//                             graph.add_uni_edge((point, (point.0 - 1, point.1)));
//                             graph.add_uni_edge((point, (point.0, point.1 + 1)));
//                             graph.add_uni_edge((point, (point.0, point.1 - 1)));
//                         },
//                         b'.' => {},
//                         _ => unreachable!()
//                     }
//                 })
//         });
//     todo!()
// }
// 
// #[cfg(test)]
// mod tests {
//     // use super::*;
// 
//     #[test]
//     fn test() {
//     }
// }
