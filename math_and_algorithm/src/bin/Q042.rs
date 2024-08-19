use num::integer::Roots;
use proconio::input;

fn main() {
    input! {n: u64}
    let result: u64 = (1..(n.sqrt() + 1)).fold(0, |sum, i| sum + i * ((n / i) * (n / i) - i * i + n / i));
    println!("{}", result);
}