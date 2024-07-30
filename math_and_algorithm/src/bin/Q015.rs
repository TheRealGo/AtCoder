use std::io;
use num::integer::gcd;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u32> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let gcd = gcd(numbers[0], numbers[1]);
    println!("{}", gcd);
}