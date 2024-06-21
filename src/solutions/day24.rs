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

use ndarray::Array1;

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

fn cross(a: Array1<f64>, b: Array1<f64>) -> Array1<f64> {
    Array1::from(vec![
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0],
    ])
}

// I am extremely disappointed that I could not use the `.dot` method like a normal person
fn dot(a: Array1<f64>, b: Array1<f64>) -> f64 {
    let mut sum = 0.;
    for i in 0..3 {
        sum += a[i] * b[i];
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{s, Array, Array1, Array2, ArrayView1};
    use optimize::{NelderMeadBuilder, Minimizer};

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
                    let (x, y, tu, tv) = intersection;
                    let in_bounds = 7.0 <= x && x <= 27.0 && 7.0 <= y && y <= 27.0;
                    let in_future = tu > 0.0 && tv > 0.0;
                    if in_bounds && in_future { n += 1; }
                }
            }
        }

        assert_eq!(n, 2);
    }

    #[test]
    fn test_matrix_stuff() {
        let loss_fn = |x: ArrayView1<f64>| {
            // f(x, y) = (1 - x) ** 2 + 100 * (y - x**2) ** 2
            (1.0 - x[0]).powi(2) + 100.0 * (x[1] - x[0].powi(2)).powi(2)
        };

        let minimizer = NelderMeadBuilder::default()
            .xtol(1e-6f64)
            .ftol(1e-6f64)
            .maxiter(50000)
            .build()
            .unwrap();

        // Set the starting guess
        let args = Array::from_vec(vec![3.0, -8.3]);
        
        // Run the optimization
        let ans = minimizer.minimize(&loss_fn, args.view());
        
        // Print the optimized values
        println!("Final optimized arguments: {}", ans);
    }

    #[test]
    fn test_part2() {
        let stones: Vec<_> = TEST
            .replace("  ", " ")
            .trim()
            .lines()
            .map(parse_line)
            .collect();
        let h0: Array2<f64> = stones.clone()
            .into_iter()
            .map(|x| [x.0[0] as f64, x.0[1] as f64, x.0[2] as f64])
            .collect::<Vec<_>>()
            .into();
        let dh: Array2<f64> = stones.clone()
            .into_iter()
            .map(|x| [x.0[0] as f64, x.0[1] as f64, x.0[2] as f64])
            .collect::<Vec<_>>()
            .into();
        println!("Shape: {:?}", h0.shape());

        let loss_fn = |x: ArrayView1<f64>| {
            // f(x, y) = sum_i ( (r(0) - h_i(0)) \cdot (dr x dh_i) )
            let r0 = x.slice(s![0..3]);
            let dr = x.slice(s![3..6]);

            let mut sum = 0.;
            for i in 0..h0.shape()[0] {
                let h0_i: Array1<f64> = h0.slice(s![i, 0..3]).to_owned();
                let dh_i: Array1<f64> = dh.slice(s![i, 0..3]).to_owned();
                let diff: Array1<f64> = r0.to_owned() - h0_i;
                let product: Array1<f64> = cross(dr.to_owned(), dh_i);
                sum += dot(diff, product);
            }
            sum
        };

        let minimizer = NelderMeadBuilder::default()
            .xtol(1e-6f64)
            .ftol(1e-6f64)
            .maxiter(50000)
            .build()
            .unwrap();

        // Set the starting guess
        let args = Array::from_vec(vec![1.; 6]);
        
        // Run the optimization
        let ans: Array1<f64> = minimizer.minimize(&loss_fn, args.view());
        
        // Print the optimized values
        println!("Final optimized arguments: {}", ans);
        assert_eq!(ans.slice(s![0..3]), Array1::from(vec![24., 13., 10.]).view());
    }
}
