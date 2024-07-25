use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let numbers: Vec<i32> = input_line.trim().split_whitespace().map(|s| s.parse().expect("Please type a number!")).collect();
    let sum: i32 = numbers.iter().sum();
    println!("{}", sum);
}