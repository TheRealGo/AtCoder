use std::io;
use std::io::BufRead;

fn main() {
    let reader = io::stdin().lock();
    let mut lines = reader.lines();

    lines.next();

    let mut an: Vec<u32> = lines.next()
        .unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    an = sort(an);

    let result = an.iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", result);
}

fn sort(mut an: Vec<u32>) -> Vec<u32> {
    if an.len() <= 1{
        return an;
    }

    let pivot: u32 = an.remove(0);
    let mut a_small: Vec<u32> = Vec::new();
    let mut a_big: Vec<u32> = Vec::new();

    for a in an {
        if a <= pivot {
            a_small.push(a);
        } else {
            a_big.push(a);
        }
    }

    let mut result = sort(a_small);
    result.push(pivot);
    result.extend(sort(a_big));
    result
}