use std::io;
use std::io::Read;
use std::io::BufRead;

fn main() {
    println!("Varius input types");
}

// 1行ずつ読み込む
fn line() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}

// スペース区切りで複数の値を読み込む
fn space() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{:?}", numbers);
}

// 複数行にわたる複数のデータを読み込む
fn lines() {
    let mut iterator = io::stdin().lock().lines();

    while let Some(Ok(line)) = iterator.next() {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        println!("{:?}", numbers);
    }
}

// 行数を受け取る -> 行ごとに処理をする
fn parse() {
    let mut lines = io::stdin().lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let n: i32 = first_line.parse().unwrap();

    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            println!("{}", line);
        }
    }
}

// 一度に文書全体を読み込む
fn parse_input() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut x = Vec::with_capacity(n);
    let mut y = Vec::with_capacity(n);

    for _ in 0..n {
        x.push(iter.next().unwrap().parse::<u32>().unwrap());
        y.push(iter.next().unwrap().parse::<u32>().unwrap());
    }

    println!("{:?}, {:?}", x, y);
}