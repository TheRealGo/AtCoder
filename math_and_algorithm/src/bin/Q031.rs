use std::io;
use std::io::BufRead;

fn main() {
    let mut lines = io::stdin().lock().lines();
    lines.next();
    let mut numbers: Vec<u64> = lines.next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut p1: u64 = 0;
    let mut p2: u64 = numbers.remove(0);
    for a in numbers {
        let experience: u64 = p2.max(p1 + a);
        p1 = p2;
        p2 = experience;
    }

    println!("{}", p2);
}