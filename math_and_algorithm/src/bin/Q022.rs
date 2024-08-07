use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let an: Vec<usize> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let target = 100000;

    let mut numbers: Vec<u64> = vec![0; target + 1];
    for a in an {
        numbers[a] += 1;
    }

    let mut count = 0;
    for i in 1..50000 {
        count += numbers[i] * numbers[target - i];
    }
    count += numbers[50000] * (numbers[50000] - 1) / 2;

    println!("{}", count);
}