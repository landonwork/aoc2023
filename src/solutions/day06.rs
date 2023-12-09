use crate::{read_input, Solutions};

fn part1(input: &[String]) -> i64 {
    let times: Vec<_> = input[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let distances: Vec<_> = input[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    times.into_iter().zip(distances).map(ways_to_win).product()
}

fn ways_to_win((time, dist): (f64, f64)) -> i64 {
    let odd = time % 2.;
    // calculate discriminant (the square root in the quadratic formula)
    let discriminant = (time * time - 4. * dist).sqrt();
    let rem = discriminant % 2.;
    if (odd == 1. && rem > 1.) || (odd == 0. && rem < 1.) {
        discriminant.ceil() as i64
    } else {
        discriminant.floor() as i64
    }
}

fn part2(input: &[String]) -> i64 {
    let time = input[0]
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<f64>()
        .unwrap();
    let dist = input[1]
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<f64>()
        .unwrap();
    ways_to_win((time, dist))
}

pub fn solve() -> Solutions {
    let input = read_input("06");
    let solution1 = part1(&input);
    let solution2 = part2(&input);
    Solutions(solution1.to_string(), solution2.to_string())
}
