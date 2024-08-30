use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! { k: usize }

    let mut graph: Vec<usize> = vec![usize::MAX; k];
    let mut queue: VecDeque<usize> = VecDeque::new();

    graph[1] = 1;
    queue.push_back(1);
    while let Some(node) = queue.pop_front() {
        if graph[(node * 10) % k] > graph[node] {
            graph[(node * 10) % k] = graph[node];
            queue.push_front((node * 10) % k);
        }

        if graph[(node + 1) % k] > graph[node] + 1 {
            graph[(node + 1) % k] = graph[node] + 1;
            queue.push_back((node + 1) % k);
        }
    }

    println!("{}", graph[0]);
}