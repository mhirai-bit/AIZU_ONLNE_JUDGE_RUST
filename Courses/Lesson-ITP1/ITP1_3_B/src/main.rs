use std::io;

fn main() {
    let nums: Vec<u32> = read();
    for (i, val) in nums.iter().enumerate() {
        println!("Case {}: {}", i+1, val);
    }
    
}


fn read() -> Vec<u32> {
    let mut input = String::new();
    let mut nums = vec![];
    while let Some(_) = io::stdin().read_line(&mut input).ok() {
        let x = match input.trim().parse::<u32>() {
            Ok(0) => return nums,
            Ok(x) => x,
            Err(_) => return nums
        };
        input.clear(); 
        nums.push(x);       
    } 
    nums
}