use crate::Day;

pub struct Day11;

impl Day for Day11 {
    async fn part1(input: String) -> String {
        part1(input.trim()).to_string()
    }

    async fn part2(input: String) -> String {
        part2(input.trim()).to_string()
    }
}

use std::collections::HashSet;

fn part1(input: &str) -> usize {
    solve::<1>(input)
}

fn part2(input: &str) -> usize {
    solve::<999999>(input)
}

fn solve<const C: usize>(input: &str) -> usize {
    let mut n_rows = 0;
    let mut n_cols = 0;
    let galaxies: HashSet<_> = input.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            n_rows += 1;
            n_cols = std::cmp::max(n_cols, line.trim().len());
            line.trim()
                .chars()
                .enumerate()
                .filter_map(move |(col, c)| (c == '#').then_some((row, col)))
        })
        .collect();
    let galaxies = expand::<C>(galaxies, n_rows, n_cols);
    let pairs = pairs(galaxies);
    pairs.into_iter()
        .map(|(gx1, gx2)| gx1.0.abs_diff(gx2.0) + gx1.1.abs_diff(gx2.1))
        .sum()
}

fn expand<const C: usize>(mut set: HashSet<(usize, usize)>, n_rows: usize, n_cols: usize) -> HashSet<(usize, usize)> {
    let rows: Vec<_> = set.iter().map(|(row, _col)| row).copied().collect();
    let cols: Vec<_> = set.iter().map(|(_row, col)| col).copied().collect();

    // expand rows
    (0..n_rows).rev().filter(|n| !rows.contains(n)).for_each(|n| {
        set = set.iter().copied().map(|(row, col)| if row > n {
            (row + C, col)
        } else {
            (row, col)
        }).collect();
    });

    // expand columns
    (0..n_cols).rev().filter(|n| !cols.contains(n)).for_each(|n| {
        set = set.iter().copied().map(|(row, col)| if col > n {
            (row, col + C)
        } else {
            (row, col)
        }).collect();
    });

    set
}

fn pairs(single: HashSet<(usize, usize)>) -> HashSet<((usize, usize), (usize, usize))> {
    let mut other = single.clone();
    single.into_iter()
        .flat_map({
            move |gx1| {
                other.remove(&gx1);
                other.iter()
                    .copied()
                    .map(move |gx2| (std::cmp::min(gx1, gx2), std::cmp::max(gx1, gx2)))
                    .collect::<Vec<_>>()
            }
        })
        .collect()
}
