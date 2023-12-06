use std::str::FromStr;

use crate::Solutions;

pub fn read_input() -> String {
    std::fs::read_to_string(format!("input/day05.txt"))
        .unwrap()
        .trim()
        .replace("\r", "")
}

#[derive(Debug)]
struct Range {
    dest: i64,
    src: i64,
    src_end: i64,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [dest, src, length] = s
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();
        Ok(Self { dest, src, src_end: src + length })
    }
}

impl Range {
    const fn translate(&self, num: i64) -> i64 {
        num - self.src + self.dest
    }

    const fn contains(&self, num: i64) -> bool {
        self.src <= num && num <= self.src_end
    }
}

fn part1(mut seeds: Vec<i64>, maps: &[&[Range]]) -> i64 {
    for map in maps.iter() {
        seeds = seeds.into_iter()
            .map(|x| map.iter()
                .find_map(|range| (range.src <= x && x <= range.src_end).then_some(range.translate(x)))
                .unwrap_or(x)
            )
            .collect();
    }
    seeds.into_iter().min().unwrap()
}

fn part2(mut seeds: Vec<(i64, i64)>, maps: &[&[Range]]) -> i64 {
    for map in maps {
        let cap = seeds.len() << 1;
        let mut old_seeds = std::mem::replace(&mut seeds, Vec::with_capacity(cap));
        while let Some((min, max)) = old_seeds.pop() {
            let seed = map.iter()
                .find_map(|range| {
                    if range.contains(min) && range.contains(max) {
                        Some((range.translate(min), range.translate(max)))
                    } else if range.contains(min) {
                        old_seeds.push((range.src_end + 1, max));
                        Some((range.translate(min), range.translate(range.src_end)))
                    } else if range.contains(max) {
                        old_seeds.push((min, range.src - 1));
                        Some((range.translate(range.src), range.translate(max)))
                    } else if min < range.src && range.src_end < max {
                        old_seeds.push((min, range.src - 1));
                        old_seeds.push((range.src_end + 1, max));
                        Some((range.translate(range.src), range.translate(range.src_end)))
                    } else {
                        None
                    }
                })
                .unwrap_or((min, max));
            seeds.push(seed);
        }
    }
    seeds.into_iter().map(|(min, _)| min).min().unwrap()
}

pub fn solve() -> Solutions {
    let input = read_input();
    let mut chunks = input.split("\n\n");
    let seeds: Vec<_> = chunks.next()
        .and_then(|x| x.strip_prefix("seeds: "))
        .unwrap()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    let maps: Vec<Vec<Range>> = chunks.map(|chunk| chunk.split("\n").skip(1).map(|line| line.parse().unwrap()).collect()).collect();
    let maps_refs = maps.iter().map(|vec| vec.as_slice()).collect::<Vec<_>>();

    let solution1 = part1(seeds.clone(), maps_refs.as_slice());
    let solution2 = part2(seeds.chunks_exact(2).map(|slice| (slice[0], slice[0] + slice[1])).collect(), maps_refs.as_slice());

    Solutions(solution1.to_string(), solution2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let mut chunks = input.split("\n\n");
        let seeds: Vec<_> = chunks.next()
            .and_then(|x| x.strip_prefix("seeds: "))
            .unwrap()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();
        let maps: Vec<Vec<Range>> = chunks.map(|chunk| chunk.split("\n").skip(1).map(|line| line.parse().unwrap()).collect()).collect();
        let maps_refs = maps.iter().map(|vec| vec.as_slice()).collect::<Vec<_>>();

        assert_eq!(
            part2(seeds.chunks_exact(2).map(|slice| (slice[0], slice[0] + slice[1])).collect(), maps_refs.as_slice()),
            46
        );
    }
}
