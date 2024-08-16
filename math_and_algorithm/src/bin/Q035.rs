use std::io;
use std::io::BufRead;

fn main() {
    let mut lines = io::stdin().lock().lines();
    let a: Vec<u64> = lines.next().unwrap().unwrap()
        .split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
    let b: Vec<u64> = lines.next().unwrap().unwrap()
        .split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();

    let x_diff: u64 = a[0].abs_diff(b[0]);
    let y_diff: u64 = a[1].abs_diff(b[1]);
    let d: u64 = x_diff * x_diff + y_diff * y_diff;

    let r_add: u64 = (a[2] + b[2]) * (a[2] + b[2]);

    let diff = a[2].abs_diff(b[2]);
    let r_subtract: u64 = diff * diff;

    if d < r_subtract {
        println!("1")
    } else if d == r_subtract {
        println!("2")
    } else if d < r_add {
        println!("3")
    } else if d == r_add {
        println!("4")
    } else if d > r_add {
        println!("5")
    }
}