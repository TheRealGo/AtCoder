use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let n: i32 = input.trim().parse().expect("Please type a number!");
    let result = n * 2 + 3;
    println!("{}", result);
}