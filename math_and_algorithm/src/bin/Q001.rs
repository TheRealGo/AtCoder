use std::io;

fn main() {
  let mut n = String::new();
  io::stdin().read_line(&mut n).expect("Failed to read line");
  let mut number: i32 = n.trim().parse().expect("Please type a number!");
  number += 5;
  println!("{}", number);
}