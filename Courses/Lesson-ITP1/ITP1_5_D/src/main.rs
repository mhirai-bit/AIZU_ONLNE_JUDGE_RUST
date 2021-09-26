//Structured Programming

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    for x in 1..=n {
        if judge(x) {
            print!(" {}", x);
        }
    }
    println!();
}

fn judge(mut x: u32) -> bool {
    if x % 3 == 0 {
        true
    } else {
        while x > 0 {
            if x % 10 == 3 {
                return true;
            } else {
                 x /= 10; 
            }
        }
        false
    }
}