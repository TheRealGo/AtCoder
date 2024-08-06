use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u32> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = numbers[0];
    let r = numbers[1];

    let r = if n / 2 < r { n - r } else { r };

    let mut c = 1;
    for i in 0..r {
        c *= n - i;
        c /= i + 1;
    }

    println!("{}", c);
}