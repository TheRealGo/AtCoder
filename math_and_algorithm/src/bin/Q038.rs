use std::io;
use std::io::BufRead;

fn main() {
    let mut lines = io::stdin().lock().lines();

    let nq: Vec<usize> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut an: Vec<u32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    for i in 1..nq[0] {
        an[i] += an[i - 1];
    }

    for line in lines.take(nq[1]) {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let l = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let r = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let result = if l == 0 { an[r] } else { an[r] - an[l - 1] };
        println!("{}", result);
    }
}