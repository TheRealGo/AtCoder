use std::io;
use std::process::exit;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let numbers: Vec<usize> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();

    let _n = numbers[0];
    let sum = numbers[1];

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let numbers: Vec<usize> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();

    let mut dp: Vec<bool> = vec![false; sum + 1];
    dp[0] = true;

    for n in numbers {
        for i in (0..sum).rev() {
            if dp[i] && i + n <= sum {
                dp[i + n] = true;
                if dp[sum] {
                    println!("Yes");
                    exit(0);
                }
            }
        }
    }
    println!("No");
}