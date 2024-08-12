use std::io;
use std::io::BufRead;

fn main() {
    let mut lines = io::stdin().lock().lines();
    let line1: Vec<usize> = lines.next()
        .unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let n = line1[0];
    let w = line1[1];

    let mut values: Vec<usize> = vec![0; w + 1];

    for _ in 0..n {
        let line: Vec<usize> = lines.next()
            .unwrap().unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let wi = line[0];
        let vi = line[1];

        for i in (wi..w).rev() {
            let index = i + 1;
            let v = values[index - wi];
            values[index] = (v + vi).max(values[index]);
        }
        values[wi] = vi.max(values[wi]);
    }

    let result = values.iter().max().unwrap();
    println!("{}", result);
}