use proconio::input;

fn main() {
    input! { n: i32, x: i32, y: i32 }

    let manhattan = n
        - if x > 0 { x } else { -x }
        - if y > 0 { y } else { -y };

    if manhattan % 2 == 0 && manhattan >= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}