use crate::Day;

pub struct Day24;

impl Day for Day24 {
    async fn part1(input: String) -> String {
        let stones: Vec<_> = input
            .replace("  ", " ")
            .trim()
            .lines()
            .map(parse_line)
            .collect();

        let mut n = 0;
        for i in 0..stones.len() - 1 {
            for j in i + 1..stones.len() {
                if let Some(intersection) = find_intersection_2d(stones[i], stones[j]) {
                    let (x, y, tu, tv) = intersection;
                    let in_bounds = 200000000000000.0 <= x && x <= 400000000000000.0 && 200000000000000.0 <= y && y <= 400000000000000.0;
                    let in_future = tu > 0.0 && tv > 0.0;
                    if in_bounds && in_future { n += 1; }
                }
            }
        }

        n.to_string()
    }
}

type Stone = ([i64; 3], [i64; 3]);

fn parse_line(line: &str) -> Stone {
    let (left, right) = line.split_once(" @ ").unwrap();
    let p: [i64; 3] = left.split(", ").map(|x| x.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap();
    let d: [i64; 3] = right.split(", ").map(|x| x.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap();
    (p, d)
}

fn find_intersection_2d(stone1: Stone, stone2: Stone) -> Option<(f64, f64, f64, f64)> {
    let ([a0, u0, _], [da, du, _]) = stone1;
    let ([b0, v0, _], [db, dv, _]) = stone2;
    let (a0, u0, da, du) = (a0 as f64, u0 as f64, da as f64, du as f64);
    let (b0, v0, db, dv) = (b0 as f64, v0 as f64, db as f64, dv as f64);
    let slope1 = du / da;
    let slope2 = dv / db;

    (slope1 != slope2).then_some({
        let x = (a0 * slope1 - b0 * slope2 + v0 - u0) / (slope1 - slope2);
        let y = (x - a0) * slope1 + u0;
        let tu = (x - a0) / da;
        let tv = (x - b0) / db;
        (x, y, tu, tv)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"#;

    #[test]
    fn test_part1() {
        let stones: Vec<_> = TEST
            .replace("  ", " ")
            .trim()
            .lines()
            .map(parse_line)
            .collect();

        let mut n = 0;
        for i in 0..stones.len() - 1 {
            for j in i + 1..stones.len() {
                if let Some(intersection) = find_intersection_2d(stones[i], stones[j]) {
                    println!("{:?}, {:?} => {:?}", stones[i], stones[j], intersection);
                    let (x, y, tu, tv) = intersection;
                    let in_bounds = 7.0 <= x && x <= 27.0 && 7.0 <= y && y <= 27.0;
                    let in_future = tu > 0.0 && tv > 0.0;
                    if in_bounds && in_future { n += 1; }
                }
            }
        }

        assert_eq!(n, 2);
    }
}
