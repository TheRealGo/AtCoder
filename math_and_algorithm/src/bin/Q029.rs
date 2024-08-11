use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u8 = input.trim().parse().unwrap();

    let mut a: u32 = 1;
    let mut b: u32 = 1;
    for _ in 1..n {
        b += a;
        a = b - a;
    }

    println!("{}", b);
}