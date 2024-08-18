use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut an: [u32; n],
        lr: [(usize, usize); q],
    }

    for i in 1..n {
        an[i] += an[i - 1];
    }

    let mut result: String = String::new();
    for (l, r) in lr {
        let out = if l - 1 == 0 { an[r - 1] } else { an[r - 1] - an[l - 2] };
        result.push_str(&format!("{}\n", out));
    }
    print!("{}", result);
}