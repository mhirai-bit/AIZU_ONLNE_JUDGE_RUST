fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    let mut nums: Vec<u32> = vec![];
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    for _ in 0..n {
        nums.push(iter.next().unwrap().parse().unwrap());
    }

    for (i, num) in nums.iter().rev().enumerate() {
        print!("{}", num);
        if i + 1 < nums.len() {
            print!(" ");
        }
    }
    println!();
}

