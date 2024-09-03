use proconio::input;
const MOD: u64 = 1000000007;
const INVERSE_3: u64 = 333333336;

fn main() {
    input! { mut n: u64 }

    let mut result: u64 = 1;
    let mut base: u64 = 4;
    while n > 0 {
        if n & 1 == 1 {
            result = (result * base) % MOD;
        }
        base = (base * base) % MOD;
        n >>= 1;
    }

    result = (result + (result - 1) * INVERSE_3) % MOD;

    println!("{}", result);
}