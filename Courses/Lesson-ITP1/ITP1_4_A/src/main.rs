fn main() {
    let (a, b) = read();
    println!("{} {} {}", (a / b) as u32,
                          a % b,
                          a / b);
}


fn read() -> (f64, f64) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a = iter.next().unwrap().parse().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    (a, b)
}