
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a = iter.next().unwrap().parse::<f64>().unwrap();
    let b = iter.next().unwrap().parse::<f64>().unwrap();
    let c = iter.next().unwrap().parse::<f64>().unwrap();
    
    let h = b * c.to_radians().sin();
    let s = a * h / 2.;
    let l = a + b + ((a - b * c.to_radians().cos()).powi(2) + h.powi(2)).sqrt();

    println!("{}\n{}\n{}", s, l, h);

}
