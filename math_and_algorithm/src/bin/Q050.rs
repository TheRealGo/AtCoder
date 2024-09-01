use proconio::input;
const MOD: u64 = 1000000007;

fn main() {
    input! { mut a: u64, mut b: u64 }

    let mut result: u64 = 1;
    while b > 0 {
        if b % 2 == 1 {
            result *= a;
            result %= MOD;
        }
        a *= a;
        a %= MOD;
        b /= 2;
    }
    println!("{}", result);
}