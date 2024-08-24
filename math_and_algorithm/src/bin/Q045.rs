use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut visited: Vec<bool> = vec![true; n + 1];
    let mut flag: Vec<bool> = vec![false; n + 1];
    let mut result: u32 = 0;
    for (a, b) in ab {
        let node: usize = a.max(b);
        if visited[node] {
            visited[node] = false;
            flag[node] = true;
            result += 1;
        } else if flag[node] {
            flag[node] = false;
            result -= 1;
        }
    }

    println!("{}", result);
}