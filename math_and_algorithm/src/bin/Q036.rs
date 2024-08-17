use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let clock: Vec<u16> = input.split_whitespace().map(|s| s.parse::<u16>().unwrap()).collect();

    let a_theta: f64 = std::f64::consts::PI * ((60 * clock[2] + clock[3]) as f64) / 360.0;
    let b_theta: f64 = std::f64::consts::PI * (clock[3] as f64) / 30.0;

    let ax: f64 = (clock[0] as f64) * a_theta.cos();
    let ay: f64 = (clock[0] as f64) * a_theta.sin();
    let bx: f64 = (clock[1] as f64) * b_theta.cos();
    let by: f64 = (clock[1] as f64) * b_theta.sin();

    let result: f64 = ((ax - bx) * (ax - bx) + (ay - by) * (ay - by)).sqrt();

    println!("{}", result);
}