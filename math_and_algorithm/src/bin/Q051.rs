use proconio::input;
const MOD: usize = 1000000007;

fn main() {
    input! {x: usize, y: usize}

    let mut factorial: usize = 1;
    let mut n: usize = 1;
    for i in 2..x + y + 1 {
        factorial = (factorial * i) % MOD;
        if i == x { n = (n * factorial) % MOD };
        if i == y { n = (n * factorial) % MOD };
    }

    let mut inverse: usize = 1;
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