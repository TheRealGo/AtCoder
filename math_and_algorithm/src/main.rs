use std::io;
use std::io::Read;
use std::io::BufRead;

fn main() {
    // memo
    println!("   u8::MAX: {}", u8::MAX);    // 255
    println!("  u16::MAX: {}", u16::MAX);   // 65535
    println!("  u32::MAX: {}", u32::MAX);   // 4294967295
    println!("  u64::MAX: {}", u64::MAX);   // 18446744073709551615
    println!(" u128::MAX: {}", u128::MAX);  // 340282366920938463463374607431768211455
    println!("usize::MAX: {}", usize::MAX); // 18446744073709551615
    println!("==================================================");
    println!("   i8::MIN: {}", i8::MIN);    //  -128
    println!("   i8::MAX: {}", i8::MAX);    //  127
    println!("  i16::MIN: {}", i16::MIN);   //  -32768
    println!("  i16::MAX: {}", i16::MAX);   //  32767
    println!("  i32::MIN: {}", i32::MIN);   //  -2147483648
    println!("  i32::MAX: {}", i32::MAX);   //  2147483647
    println!("  i64::MIN: {}", i64::MIN);   //  -9223372036854775808
    println!("  i64::MAX: {}", i64::MAX);   //  9223372036854775807
    println!(" i128::MIN: {}", i128::MIN);  //  -170141183460469231731687303715884105728
    println!(" i128::MAX: {}", i128::MAX);  //  170141183460469231731687303715884105727
    println!("isize::MIN: {}", isize::MIN); //  -9223372036854775808
    println!("isize::MAX: {}", isize::MAX); //  9223372036854775807
    println!("==================================================");
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