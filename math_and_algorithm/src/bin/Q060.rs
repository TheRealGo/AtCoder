use proconio::input;

fn main() {
    input! { n: usize }

    if n % 4 != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}