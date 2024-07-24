use std::io;
use num::integer::lcm;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let numbers: Vec<i32> = input.trim()
                                 .split_whitespace()
                                 .map(|s| s.parse().expect("Please type a number!"))
                                 .collect();
    let n: i32 = numbers[0];
    let x: i32 = numbers[1];
    let y: i32 = numbers[2];

    let result = n / x + n / y - n / lcm(x, y);
    println!("{}", result);
}