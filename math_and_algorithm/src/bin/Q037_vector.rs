use std::io;
use std::io::BufRead;

fn main() {
    let mut points: Vec<(i128, i128)> = io::stdin()
        .lock()
        .lines()
        .take(4)
        .map(|line| {
            let line = line.unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<i128>().unwrap(),
                iter.next().unwrap().parse::<i128>().unwrap(),
            )
        })
        .collect();

    if points[0] > points[1] {
        points.swap(0, 1);
    }
    if points[2] > points[3] {
        points.swap(2, 3);
    }

    let v01: (i128, i128) = (points[1].0 - points[0].0, points[1].1 - points[0].1);
    let v02: (i128, i128) = (points[2].0 - points[0].0, points[2].1 - points[0].1);
    let v03: (i128, i128) = (points[3].0 - points[0].0, points[3].1 - points[0].1);
    let v21: (i128, i128) = (points[1].0 - points[2].0, points[1].1 - points[2].1);
    let v23: (i128, i128) = (points[3].0 - points[2].0, points[3].1 - points[2].1);

    let v102: i128 = cross_product(v01, v02, 1);
    let v103: i128 = cross_product(v01, v03, 1);

    let flg: bool;
    if v102 + v103 == 0 {
        flg = points[0].max(points[2]) <= points[1].min(points[3]);
    } else {
        flg = v102 * v103 <= 0 && cross_product(v23, v02, -1) * cross_product(v23, v21, 1) <= 0;
    }

    let result: String = String::from("NYoe s");
    println!("{}", result.chars().skip(flg as usize).step_by(2).collect::<String>());
}

fn cross_product(a: (i128, i128), b: (i128, i128), r: i128) -> i128 {
    r * (a.0 * b.1 - a.1 * b.0)
}