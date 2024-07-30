use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: u64 = input.trim().parse().unwrap();

    let mut factors: Vec<u64> = Vec::new();

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    while n % 3 == 0 {
        factors.push(3);
        n /= 3;
    }

    let mut factor: u64 = 5;
    let mut add: u8 = 2;
    while factor * factor < n {
        while n % factor == 0 {
            factors.push(factor);
            n /= factor;
        }
        factor += add as u64;
        add = 6 - add;
    }

    if n != 1 {
        factors.push(n);
    }

    let output = factors.iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
}