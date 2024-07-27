use std::io;
use num_bigint::BigUint;
use num_traits::{Zero, One};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let numbers: Vec<u16> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();

    let _n = numbers[0];
    let sum = numbers[1];

    let mut bit_flag = BigUint::one();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let numbers: Vec<u16> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();

    for i in numbers.iter() {
        bit_flag |= &bit_flag << i;
    };

    bit_flag &= BigUint::one() << sum;

    let result = if bit_flag != BigUint::zero() {
        "Yes"
    } else {
        "No"
    };

    println!("{}", result);
}