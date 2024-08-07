use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: f64 = input.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let b_sum: u32 = input.trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .sum();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let r_sum: u32 = input.trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .sum();

    let result = (b_sum + r_sum) as f64 / n;

    println!("{}", result);
}