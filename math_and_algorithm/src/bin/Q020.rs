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

    println!("{}", dp(an, 1000, 5));
}

fn dp(an: Vec<usize>, target: usize, m: usize) -> u16 {
    let mut memo: Vec<Vec<u16>> = vec![vec![0; target + 1]; m + 1];
    memo[0][0] = 1;
    for a in an {
        for step in (1..=m).rev() {
            for i in (a..=target).rev() {
                memo[step][i] += memo[step - 1][i - a];
            }
        }
    }
    memo[m][target]
}