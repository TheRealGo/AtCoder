use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut counts: Vec<u32> = vec![0; 4];
    for an in input.trim().split_whitespace() {
        match an {
            "100" => counts[0] += 1,
            "200" => counts[1] += 1,
            "300" => counts[2] += 1,
            "400" => counts[3] += 1,
            _ => {}
        }
    }

    let result = counts[0] * counts[3] + counts[1] * counts[2];
    println!("{}", result);
}