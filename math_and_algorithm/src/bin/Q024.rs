use std::io;
use std::io::BufRead;

fn main() {
    let reader = io::stdin().lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let n: u8 = first_line.parse().unwrap();

    let mut result: f64 = 0.0;
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let vec: Vec<f64> = line.trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        result += vec[1] / vec[0]
    }

    println!("{}", result);
}