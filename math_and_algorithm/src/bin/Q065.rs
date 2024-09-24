use proconio::input;

fn main() {
    input! { h: u64, w: u64 }
    if h == 1 || w == 1 {
        println!("1");
    } else {
        let rectangle: u64 = h * w;
        println!("{}", rectangle / 2 + rectangle % 2);
    }
}