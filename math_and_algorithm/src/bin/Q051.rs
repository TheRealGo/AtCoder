use proconio::input;
const MOD: u64 = 1000000007;

fn main() {
    input! {x: u64, y: u64}

    let mut factorial: u64 = 1;
    let mut n: u64 = 1;
    for i in 2..x + y + 1 {
        factorial = (factorial * i) % MOD;
        if i == x { n = (n * factorial) % MOD };
        if i == y { n = (n * factorial) % MOD };
    }

    let mut inverse: u64 = 1;
    let mut p = MOD - 2;
    while p > 0 {
        if p & 1 == 1 {
            inverse = (inverse * n) % MOD;
        }
        n = (n * n) % MOD;
        p >>= 1;
    }

    let result = (factorial * inverse) % MOD;
    println!("{}", result);
}