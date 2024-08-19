use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        lrx: [(usize, usize, i32); q],
    }

    let mut area: Vec<i32> = vec![0; n - 1];

    for (l, r, x) in lrx {
        if l > 1 {
            area[l - 2] -= x;
        }
        if r < n {
            area[r - 1] += x;
        }
    }

    let result: String = area.iter()
        .map(|&z| {
            if z < 0 { "<" } else if z > 0 { ">" } else { "=" }
        })
        .collect();

    println!("{}", result);
}