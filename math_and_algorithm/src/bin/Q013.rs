use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    let mut divisor = Vec::new();

    let mut i = 1;
    while i * i < n {
        if n % i == 0 {
            divisor.push(i);
            divisor.push(n / i);
        }
        i += 1
    }
    if i * i == n {
        divisor.push(i);
    }

    let output = divisor.iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", output);
}