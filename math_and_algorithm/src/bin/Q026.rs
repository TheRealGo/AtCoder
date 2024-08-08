use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let result = n as f64 * (1..=n).fold(0.0, |sum, i| sum + 1.0 / i as f64);

    println!("{}", result);
}