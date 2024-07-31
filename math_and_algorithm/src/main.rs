fn main() {
    println!("Hello, world!");
}

// 1行ずつ読み込む
use std::io::{self, BufRead};

fn line() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}

// スペース区切りで複数の値を読み込む
// use std::io;

fn space() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{:?}", numbers);
}

// 複数行にわたる複数のデータを読み込む
// use std::io::{self, BufRead};

fn lines() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    while let Some(Ok(line)) = iterator.next() {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        println!("{:?}", numbers);
    }
}

// 高度な入力パース
// use std::io::{self, BufRead};

fn parse() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let n: i32 = first_line.parse().unwrap();

    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            println!("{}", line);
        }
    }
}