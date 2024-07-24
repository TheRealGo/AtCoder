use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line.");

    let mut an = String::new();
    io::stdin().read_line(&mut an).expect("Failed to read line.");

    let an_vec: Vec<i32> = an.trim()
                             .split_whitespace()
                             .map(|s| s.parse().expect("Please type a number!"))
                             .collect();

    let sum: i32 = an_vec.iter().sum();
    let result = sum % 100;

    println!("{}", result);
}