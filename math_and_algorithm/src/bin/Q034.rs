use std::io;
use std::io::BufRead;

fn main() {
    let mut lines = io::stdin().lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    let mut points: Vec<(u64, u64)> = Vec::with_capacity(n);

    for line in lines.take(n) {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        points.push((x, y));
    }
    points.sort_unstable();

    let mut d = u64::MAX;

    for i in 0..n {
        for j in i + 1..n {
            let mut m = distance(points[i].0, points[j].0);
            if d < m {
                break;
            }
            m += distance(points[i].1, points[j].1);
            d = m.min(d);
        }
    }

    let result: f64 = (d as f64).sqrt();
    println!("{}", result);
}

fn distance(x: u64, y: u64) -> u64 {
    let diff = x.abs_diff(y);
    diff * diff
}