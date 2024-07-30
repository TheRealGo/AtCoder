use std::io;
use std::process::exit;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    if n < 4 {
        println!("Yes");
        return;
    } else if n % 2 == 0 {
        println!("No");
        return;
    } else if n % 3 == 0 {
        println!("No");
        return;
    }

    let mut factor: usize = 5;
    let mut add: u8 = 2;
    while factor * factor < n {
        if n % factor == 0 {
            println!("No");
            exit(0);
        }
        factor += add as usize;
        add = 6 - add;
    }
    println!("Yes");
}