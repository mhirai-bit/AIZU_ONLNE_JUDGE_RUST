use std::io::{Read, Write};

fn main() {
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut count = 0u32;

    for _ in 0..n {
        let a: i32 = iter.next().unwrap().parse().unwrap();
        if is_prime(a) {
            count+=1;
        }
    }

    write!(out, "{}\n", count).unwrap();
}

fn is_prime(a: i32) -> bool {
    if a==1 {
        return false;
    }else if a<4 {
        return true;
    }else if (a&1) == 0 {
        return false;
    }
    let mut i = 3;
    while i*i<=a {
        if a%i==0 {
            return false;
        }
        i+=2;
    }
    true
}
