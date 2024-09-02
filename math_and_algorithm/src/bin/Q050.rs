use proconio::input;
const MOD: u64 = 1000000007;

fn main() {
    input! { mut a: u64, mut b: u64 }

    let mut result: u64 = 1;
    while b > 0 {
        if b & 1 == 1 {
            result = (result * a) % MOD;
        }
        a = (a * a) % MOD;
        b >>= 1;
    }
    println!("{}", result);
}