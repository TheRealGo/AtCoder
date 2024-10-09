use proconio::input;

fn main() {
    input! { n: u64 , k: u64 }
    let result: u64 = (n - k) * (n - k) * (n + 2 * k) + (n - k) * (3 * k - 1);
    println!("{}", result);
}