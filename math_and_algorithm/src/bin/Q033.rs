use std::io;
use std::io::BufRead;

fn main() {
    let mut lines = io::stdin().lock().lines();

    let a: [i64; 2] = parse_line(&lines.next().unwrap().unwrap());
    let b: [i64; 2] = parse_line(&lines.next().unwrap().unwrap());
    let c: [i64; 2] = parse_line(&lines.next().unwrap().unwrap());

    let ab: [i64; 2] = sub(&b, &a);
    let ac: [i64; 2] = sub(&c, &a);
    let bc: [i64; 2] = sub(&c, &b);

    if dot(&ab, &bc) > 0 {
        println!("{}", distance(&ab));
    } else if dot(&ac, &bc) < 0 {
        println!("{}", distance(&ac));
    } else {
        let result: f64 = (ab[0] * ac[1] - ab[1] * ac[0]).abs() as f64 / distance(&bc);
        println!("{}", result);
    }
}

fn parse_line(line: &str) -> [i64; 2] {
    let mut iter = line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    [iter.next().unwrap(), iter.next().unwrap()]
}

fn sub(v1: &[i64; 2], v2: &[i64; 2]) -> [i64; 2] {
    [v1[0] - v2[0], v1[1] - v2[1]]
}

fn dot(v1: &[i64; 2], v2: &[i64; 2]) -> i64 {
    v1[0] * v2[0] + v1[1] * v2[1]
}

fn distance(v: &[i64; 2]) -> f64 {
    ((v[0] * v[0] + v[1] * v[1]) as f64).sqrt()
}