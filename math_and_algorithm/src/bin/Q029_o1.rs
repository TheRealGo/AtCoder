use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let sqrt_5 = 5.0_f64.sqrt();
    let alpha = (1.0 + sqrt_5) / 2.0;
    let beta = (1.0 - sqrt_5) / 2.0;
    let result = ((alpha.powi(n + 1) - beta.powi(n + 1)) / sqrt_5).round();
    println!("{}", result);
}