use proconio::input;

fn main() {
    input! { n: u32 }
    if n & 1 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}