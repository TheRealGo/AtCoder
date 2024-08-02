use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut counts: Vec<u64> = vec![0; 3];
    for an in input.trim().split_whitespace() {
        match an {
            "1" => counts[0] += 1,
            "2" => counts[1] += 1,
            "3" => counts[2] += 1,
            _ => {}
        }
    }

    let result = (counts[0] * (counts[0] - 1) + counts[1] * (counts[1] - 1) + counts[2] * (counts[2] - 1)) / 2;
    println!("{}", result);
}