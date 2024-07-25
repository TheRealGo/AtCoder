use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let numbers: Vec<i32> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();
    let n = numbers[0];
    let s = numbers[1];

    let result = if s < n {
        (s - 1) * s / 2
    } else if n <= s && s < 2 * n {
        let a = 2 * n - s;
        n.pow(2) - (a + 1) * a / 2
    } else {
        n.pow(2)
    };

    println!("{result}");
}