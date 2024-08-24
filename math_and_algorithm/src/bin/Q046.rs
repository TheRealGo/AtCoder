use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
        r: usize,
        _c: usize,
        s: (usize, usize),
        g: (usize, usize),
        map: [String; r],
    }

    let mut map: Vec<Vec<u16>> = map.iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '#' => u16::MAX,
                    '.' => 0,
                    _ => unreachable!(),
                }).collect()
        }).collect();

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(s);

    let directions: Vec<(usize, usize)> = vec![(0, 1), (1, 2), (2, 1), (1, 0)];
    while let Some((y, x)) = queue.pop_front() {
        let step: u16 = map[y - 1][x - 1] + 1;
        for (dy, dx) in &directions {
            let py = y - 2 + dy;
            let px = x - 2 + dx;
            if map[py][px] == 0 {
                map[py][px] = step;
                queue.push_back((py + 1, px + 1));
            }
        }
    }
    println!("{}", map[g.0 - 1][g.1 - 1]);
}