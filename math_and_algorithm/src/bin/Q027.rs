use std::io;
use std::io::BufRead;

fn main() {
    let reader = io::stdin().lock();
    let mut lines = reader.lines();

    lines.next();

    let mut an: Vec<u32> = lines.next()
        .unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    an.sort_unstable();

    let result = an.iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", result);
}