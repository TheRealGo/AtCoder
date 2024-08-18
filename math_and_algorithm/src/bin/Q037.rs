use std::io;
use std::io::BufRead;

fn main() {
    let points: Vec<(i128, i128)> = io::stdin()
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

    let l11 = cross_product(points[0], points[1], points[2]);
    let l12 = cross_product(points[0], points[1], points[3]);
    let l21 = cross_product(points[2], points[3], points[0]);
    let l22 = cross_product(points[2], points[3], points[1]);

    if l11 == 0 && l12 == 0 {
        let l1_max = points[0].max(points[1]);
        let l1_min = points[0].min(points[1]);
        let l2_max = points[2].max(points[3]);
        let l2_min = points[2].min(points[3]);
        if l1_max < l2_min || l1_min > l2_max {
            println!("No");
        } else {
            println!("Yes");
        };
    } else if l11 * l12 <= 0 && l21 * l22 <= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn cross_product(a: (i128, i128), b: (i128, i128), c: (i128, i128)) -> i128 {
    (a.0 - b.0) * (c.1 - b.1) - (a.1 - b.1) * (c.0 - b.0)
}