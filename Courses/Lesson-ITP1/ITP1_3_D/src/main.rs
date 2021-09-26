//How Many Divisors?

use std::io;

fn main() {
    let (a, b, c) = read();
    let mut devisers = vec![];
    for i in a..=b {
        if c % i == 0 {
            devisers.push(i);
        }
    }
    println!("{}", devisers.len());
}


fn read() -> (u32, u32, u32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let a = iter.next().unwrap().parse().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    let c = iter.next().unwrap().parse().unwrap();
    (a, b, c)
}