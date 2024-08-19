use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [u64; n - 1],
        m: usize,
        bm: [usize; m],
    }

    let mut distance: Vec<u64> = Vec::with_capacity(n);
    distance.push(0);
    for i in 0..n - 1 {
        distance.push(distance[i] + an[i]);
    }

    let mut total_distance: u64 = 0;
    for i in 1..m {
        total_distance += if bm[i - 1] < bm[i] {
            distance[bm[i] - 1] - distance[bm[i - 1] - 1]
        } else {
            distance[bm[i - 1] - 1] - distance[bm[i] - 1]
        };
    }

    println!("{}", total_distance);
}