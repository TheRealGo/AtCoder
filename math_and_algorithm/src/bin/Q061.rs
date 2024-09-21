use proconio::input;

fn main() {
    input! { n: u64 }
    if (n + 1) & n != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}