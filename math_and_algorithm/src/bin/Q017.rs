use std::io;
use num::integer::lcm;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u64> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let lcm = numbers.into_iter().reduce(|a, b| lcm(a, b)).unwrap();
    println!("{}", lcm);
}