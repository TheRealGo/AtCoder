use std::io;

fn main() {
    let mut an = String::new();
    io::stdin().read_line(&mut an).expect("Failed to read line");
    let trimed_an = an.trim();
    let splited_an = trimed_an.split_whitespace();
    let parsed_an = splited_an.map(|s| s.parse().expect("Please type a number!"));
    let an_vec: Vec<i32> = parsed_an.collect();
    println!("{}", an_vec.iter().product::<i32>());
}