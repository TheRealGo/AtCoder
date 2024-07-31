use std::io;
use num::integer::gcd;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u64> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let gcd = numbers.into_iter().reduce(|a, b| gcd(a, b)).unwrap();
    println!("{}", gcd);
}