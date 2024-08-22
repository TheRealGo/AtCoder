use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut edges: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut visited = HashSet::new();
    dfs(&edges, 1, &mut visited);

    let is_connected = visited.len() == n;

    println!("The graph is{}connected.", if is_connected { " " } else { " not " });
}

fn dfs(edges: &[Vec<usize>], target_node: usize, visited: &mut HashSet<usize>) {
    if !visited.insert(target_node) {
        return;
    }
    for &i in &edges[target_node] {
        dfs(edges, i, visited)
    }
}