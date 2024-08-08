use std::io;
use std::io::BufRead;

fn main() {
    let reader = io::stdin().lock();
    let mut lines = reader.lines();

    lines.next();

    let a: u64 = lines.next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .sum();
    let b: u64 = lines.next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .sum();

    let result: f64 = (a + 2 * b) as f64 / 3.0;
    println!("{}", result);
}