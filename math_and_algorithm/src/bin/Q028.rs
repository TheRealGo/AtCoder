use std::io;
use std::io::BufRead;

fn main() {
    let mut lines = io::stdin().lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let numbers: Vec<i32> = lines.next()
        .unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut costs: Vec<i32> = vec![0; n];
    costs[1] = (numbers[0] - numbers[1]).abs();
    for i in 2..n {
        let c1 = costs[i - 1] + (numbers[i] - numbers[i - 1]).abs();
        let c2 = costs[i - 2] + (numbers[i] - numbers[i - 2]).abs();
        costs[i] = c1.min(c2);
    }

    println!("{}", costs[n - 1]);
}