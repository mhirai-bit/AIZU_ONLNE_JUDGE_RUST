use std::f64::consts::PI;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let r = input.trim().parse::<f64>().unwrap();
    println!("{} {}", PI * r * r, 2.0 * PI *r);
}
