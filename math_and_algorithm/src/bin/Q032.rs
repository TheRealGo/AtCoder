use std::io;
use std::io::BufRead;

fn main() {
    let mut lines = io::stdin().lock().lines();
    let line1: Vec<u64> = lines.next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let numbers: Vec<u64> = lines.next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if numbers.contains(&line1[1]) {
        println!("Yes");
    } else {
        println!("No");
    }
}