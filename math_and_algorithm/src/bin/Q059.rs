use proconio::input;

fn main() {
    input! { n: usize }

    println!("{}", [6, 2, 4, 8][n % 4]);
}