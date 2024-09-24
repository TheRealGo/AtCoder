use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [usize; n],
    }

    let mut route: Vec<usize> = Vec::new();
    let mut unvisited: Vec<bool> = vec![true; n + 1];

    let mut current: usize = 1;
    let mut next = a[current - 1];
    route.push(current);
    unvisited[current] = false;
    k -= 1;

    while k > 0 && unvisited[next] {
        current = next;
        next = a[current - 1];
        route.push(current);
        unvisited[current] = false;
        k -= 1;
    }

    if k == 0 {
        println!("{}", next);
    } else {
        let loop_0: usize = route.iter().position(|&x| x == next).unwrap();
        let loop_route = &route[loop_0..];
        k %= loop_route.len();
        println!("{}", loop_route[k]);
    }
}