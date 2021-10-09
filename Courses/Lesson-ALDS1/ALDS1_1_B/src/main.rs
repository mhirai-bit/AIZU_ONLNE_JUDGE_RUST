fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let mut a = iter.next().unwrap().parse::<u64>().unwrap();
    let mut b = iter.next().unwrap().parse::<u64>().unwrap();

    let mut remainder = 1;

    while remainder != 0 {
        if a > b {
            remainder = a % b;
            a = b;
            b = remainder;
        } else {
            remainder = b % a;
            b = a;
            a = remainder;
        }
    }
    println!("{}", if a > b {a} else {b});    
}
