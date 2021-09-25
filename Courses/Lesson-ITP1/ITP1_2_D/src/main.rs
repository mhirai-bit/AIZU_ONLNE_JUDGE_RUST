use std::io;

fn main() {
    let (W, H, x, y, r) = input();

    if (x + r) <= W && x >= 0 && (y + r) <= H && y >= 0{ 
        println!("Yes");
    } else {
        println!("No");
    }
}


fn input() -> (i32, i32, i32, i32, i32) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();

    let W = iter.next().unwrap().parse().unwrap();
    let H = iter.next().unwrap().parse().unwrap();
    let x = iter.next().unwrap().parse().unwrap();
    let y = iter.next().unwrap().parse().unwrap();
    let r = iter.next().unwrap().parse().unwrap();

    (W, H, x, y, r)
}