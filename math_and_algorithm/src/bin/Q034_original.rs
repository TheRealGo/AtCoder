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

    let distance_square: u64 = nearest_pair(&mut points);

    println!("{}", (distance_square as f64).sqrt());
}

fn distance(a: &(u64, u64), b: &(u64, u64)) -> u64 {
    let x_distance = a.0.abs_diff(b.0);
    let y_distance = a.1.abs_diff(b.1);
    x_distance * x_distance + y_distance * y_distance
}

fn nearest_pair(points: &[(u64, u64)]) -> u64 {
    let n = points.len();
    if n <= 3 {
        let mut d = u64::MAX;
        for i in 0..n {
            for j in i + 1..n {
                d = d.min(distance(&points[i], &points[j]));
            }
        }
        return d;
    }

    let mid_index: usize = n / 2;
    let mid_x: u64 = points[mid_index].0;

    let d_left: u64 = nearest_pair(&points[..mid_index]);
    let d_right: u64 = nearest_pair(&points[mid_index..]);
    let mut d: u64 = d_left.min(d_right);

    let mut strip: Vec<&(u64, u64)> = points.iter()
        .filter(|&&(x, _)| x.abs_diff(mid_x) < d)
        .collect();
    strip.sort_unstable_by_key(|&&(_, y)| y);

    for i in 0..strip.len() {
        for j in i + 1..strip.len() {
            if strip[i].1.abs_diff(strip[j].1) >= d {
                break;
            }
            d = d.min(distance(&strip[i], &strip[j]));
        }
    }
    d
}