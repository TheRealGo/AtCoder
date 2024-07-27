use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut flag: Vec<bool> = vec![true; n + 1];
    for i in 2..=n {
        if flag[i] {
            println!("{}", i);
            for j in (i * i..=n).step_by(i) {
                if flag[j] {
                    flag[j] = false;
                }
            }
        }
    }
}