use std::io;

fn main() {
    let mut nums = read();
    for v in nums.iter_mut() {
        v.sort();
        println!("{} {}", v[0], v[1]);
    }
}

fn read() -> Vec<Vec<u32>> {
    let mut input = String::new();
    let mut nums: Vec<Vec<u32>> = vec![];
    while let Some(_) = io::stdin().read_line(&mut input).ok(){
        let inner_vec: Vec<u32> = input
                                    .trim()
                                    .split_whitespace()
                                    .map(|c| c.parse().unwrap())
                                    .collect();
        if inner_vec[0] == 0 && inner_vec[1] == 0 {
            return nums;
        }
        nums.push(inner_vec);
        input.clear();
    }
    nums
}