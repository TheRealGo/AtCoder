use std::io;
use std::io::BufRead;

fn main() {
    let mut lines = io::stdin().lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    let mut points: Vec<(u64, u64)> = Vec::with_capacity(n);

    for _ in 0..n {
        let line: Vec<u64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        points.push((line[0], line[1]));
    }
    points.sort_by_key(|&(x, _)| x);

    let mut d = u64::MAX;

    for i in 0..n {
        for j in i + 1..n {
            let mut m = distance(points[i].0, points[j].0);
            if d < m {
                break;
            } else {
                m += distance(points[i].1, points[j].1);
                d = m.min(d);
            }
        }
    }

    let result: f64 = (d as f64).sqrt();
    println!("{}", result);
}

fn distance(x: u64, y: u64) -> u64 {
    let diff = if x > y { x - y } else { y - x };
    diff * diff
}