fn main() {
    loop {
        match read() {
            (_, '?', _) => break,
            (a, '+', b) => println!("{}", a + b),
            (a, '-', b) => println!("{}", a - b),
            (a, '*', b) => println!("{}", a * b),
            (a, '/', b) => println!("{}", a / b),
            _ => ()
        }
    }
}


fn read() -> (i32, char, i32) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a = iter.next().unwrap().parse().unwrap();
    let op = iter.next().unwrap().parse().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    (a, op, b)
}