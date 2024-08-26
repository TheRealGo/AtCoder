use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut map: Vec<Vec<usize>> = vec![Vec::with_capacity(m); n + 1];
    for (a, b) in ab {
        map[a].push(b);
        map[b].push(a);
    }

    let mut group: Vec<i8> = vec![-1; n + 1];
    let mut queue: VecDeque<usize> = VecDeque::with_capacity(n);
    for root_node in 1..n + 1 {
        if group[root_node] != -1 {
            continue;
        }

        group[root_node] = 0;
        queue.push_back(root_node);
        while let Some(node) = queue.pop_front() {
            for &neighbor in &map[node] {
                if group[neighbor] == -1 {
                    group[neighbor] = 1 - group[node];
                    queue.push_back(neighbor);
                } else if group[neighbor] == group[node] {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}