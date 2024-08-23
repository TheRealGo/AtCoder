use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut mapping: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for (a, b) in ab {
        mapping[a].push(b);
        mapping[b].push(a);
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut steps: Vec<i32> = vec![-1; n + 1];
    steps[1] = 0;

    queue.push_back(1);
    while let Some(node) = queue.pop_front() {
        for &i in &mapping[node] {
            if steps[i] == -1 {
                steps[i] = steps[node] + 1;
                queue.push_back(i);
            }
        }
    }

    let mut result: String = String::new();
    for &s in &steps[1..] {
        result.push_str(&format!("{}\n", s));
    }
    println!("{}", result);
}