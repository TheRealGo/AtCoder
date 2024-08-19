use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut time: Vec<i32> = vec![0; t + 1];
    for (l, r) in lr {
        time[l] += 1;
        time[r] -= 1;
    }

    let mut result: String = String::new();
    let mut e: i32 = 0;
    for i in 0..t {
        e += time[i];
        result.push_str(&format!("{}\n", e));
    }
    println!("{}", result);
}