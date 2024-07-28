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

    let mut a = 5;
    while a * a < n {
        if n % a == 0 || n % (a + 2) == 0 {
            println!("No");
            exit(0);
        }
        a += 6;
    }
    println!("Yes");
}