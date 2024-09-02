use proconio::input;
const MOD: u64 = 1000000007;

fn main() {
    input! {mut x: u64, mut y: u64}

    if 2 * y < x || 2 * x < y {
        println!("0");
        return;
    }

    let x_dash = 2 * y - x;
    let y_dash = 2 * x - y;

    if x_dash % 3 != 0 || y_dash % 3 != 0 {
        println!("0");
        return;
    }

    let x = x_dash / 3;
    let y = y_dash / 3;

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