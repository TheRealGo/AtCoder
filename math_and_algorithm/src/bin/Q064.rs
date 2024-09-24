use proconio::input;

fn main() {
    input! {
        n: u16,
        k: u16,
        a: [u16; n],
    }

    let sum: u16 = a.iter().sum();
    if k < sum {
        println!("No");
    } else if k - sum & 1 == 1 {
        println!("No");
    } else {
        println!("Yes");
    }
}