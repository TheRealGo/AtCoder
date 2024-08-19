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

    let mut result: String = String::new();
    for ai in area {
        result.push_str(if ai < 0 { "<" } else if ai > 0 { ">" } else { "=" });
    }

    println!("{}", result);
}