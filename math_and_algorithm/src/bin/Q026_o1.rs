use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: f32 = input.trim().parse().unwrap();
    let n_inv: f32 = 1.0 / n;
    let n_inv2: f32 = n_inv * n_inv;
    let n_inv4: f32 = n_inv2 * n_inv2;
    const EGAMMA: f32 = 0.577215664901532860606512090082402431;

    let result = n * (n.ln() + (n_inv / 2.0 - n_inv2 / 12.0 + n_inv4 / 120.) + EGAMMA);

    println!("{}", result);
}