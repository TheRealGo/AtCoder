use std::io;
use std::io::BufRead;

fn main() {
    let mut lines = io::stdin().lock().lines();
    let line1: Vec<u64> = lines.next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut numbers: Vec<u64> = lines.next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    numbers.sort_unstable();
    println!("{}", search(line1[1], &numbers));
}

fn search(x: u64, numbers: &[u64]) -> String {
    if numbers.len() == 1 {
        return if numbers[0] == x {
            "Yes".to_string()
        } else {
            "No".to_string()
        }
    }

    let i = numbers.len() / 2;
    if x < numbers[i] {
        search(x, &numbers[..i])
    } else {
        search(x, &numbers[i..])
    }
}