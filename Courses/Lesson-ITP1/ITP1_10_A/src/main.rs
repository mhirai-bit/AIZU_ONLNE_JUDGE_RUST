fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let x1 = iter.next().unwrap().parse::<f64>().unwrap();
    let y1 = iter.next().unwrap().parse::<f64>().unwrap();
    let x2 = iter.next().unwrap().parse::<f64>().unwrap();
    let y2 = iter.next().unwrap().parse::<f64>().unwrap();

    let a = x2 - x1;
    let b = y2 - y1;
    let c = (a*a + b*b).sqrt();
    println!("{}", c);
}
