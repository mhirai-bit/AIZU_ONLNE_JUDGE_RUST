//Min, Max and Sum

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse().unwrap();
    let mut num = String::new();
    let mut nums: Vec<i64> = vec![];
    std::io::stdin().read_line(&mut num).unwrap();
    let mut iter = num.split_whitespace();
    for _ in 0..n {
        let num = iter.next().unwrap().parse().unwrap();
        nums.push(num);
    }
    println!("{} {} {}", nums.iter().min().unwrap(),
                         nums.iter().max().unwrap(),
                         nums.iter().fold(0, |sum, a| sum + a));
}
