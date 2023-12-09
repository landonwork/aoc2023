use num::{Integer, BigInt};
use crate::{read_input, Solutions};

fn part1(sequences: &[Vec<i64>]) -> BigInt {
    sequences.iter()
        .map(|seq| {
            let mut current_seq = seq;
            let mut degrees = vec![];
            while current_seq.iter().any(|&x| x != 0) {
                let degree = differentiate(current_seq);
                degrees.push(degree);
                current_seq = &degrees[degrees.len()-1].1;
            }

            let mut zeros = current_seq.clone();
            zeros.push(0);
            while let Some((constant, _)) = degrees.pop() {
                zeros = integrate(constant, &zeros);
            }

            Into::<BigInt>::into(zeros.pop().unwrap())
        })
        .sum()
}

fn differentiate<I: Integer + Copy>(seq: &[I]) -> (I, Vec<I>) {
    let constant = seq[0];
    let new = seq.iter()
        .skip(1)
        .scan(constant, |acc, y| {
            let diff = *y - *acc;
            *acc = *y;
            Some(diff)
        })
        .collect();
    (constant, new)
}

fn integrate(constant: i64, seq: &[i64]) -> Vec<i64> {
    let mut new = Vec::with_capacity(seq.len()+1);
    new.push(constant);
    new.extend(seq.iter().scan(constant, |acc, x| { *acc += x; Some(*acc) }));
    new
}

fn part2(sequences: &[Vec<i64>]) -> BigInt {
    sequences.iter()
        .map(|seq| {
            let mut current_seq = seq;
            let mut degrees = vec![];
            while current_seq.iter().any(|&x| x != 0) {
                let degree = differentiate(current_seq);
                degrees.push(degree);
                current_seq = &degrees[degrees.len()-1].1;
            }

            let mut ans = 0;
            while let Some((constant, _)) = degrees.pop() {
                ans = constant - ans;
            }

            Into::<BigInt>::into(ans)
        })
        .sum()
}

pub fn solve() -> Solutions {
    let lines = read_input("09").into_iter();
    let sequences: Vec<Vec<i64>> = lines.map(|line| line.split(" ").map(|x| x.parse().unwrap()).collect()).collect();
    let solution1 = part1(&sequences);
    let solution2 = part2(&sequences);

    Solutions(solution1.to_string(), solution2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_differentiate() {
        let orig = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let test_deriv = vec![1, 1, 1, 1, 1, 1, 1];
        let (constant, deriv) = differentiate(&orig);
        assert_eq!(constant, 1);
        assert_eq!(test_deriv, deriv);
    }

    #[test]
    fn test_integrate() {
        let orig = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let test_deriv = vec![1, 1, 1, 1, 1, 1, 1];
        let constant = 1;
        let integral = integrate(constant, &test_deriv);
        assert_eq!(orig, integral);
    }
}
