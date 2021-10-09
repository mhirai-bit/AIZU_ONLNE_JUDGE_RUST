fn main() {
    let mut len = String::new();
    std::io::stdin().read_line(&mut len).unwrap();
    let mut nums = String::new();
    std::io::stdin().read_line(&mut nums).unwrap();
    
    let mut nums: Vec<u64> = nums.trim().split_whitespace()
                            .map(|c| c.parse::<u64>().unwrap()).collect();
    for (i, num) in nums.iter().enumerate() {
        print!("{}{}", num, if i == nums.len()-1 {'\n'} else {' '}); 
    }   
    
    for i in 1..nums.len() {
        let mut j: i64 = i as i64 - 1i64;
        let key = nums[i];
        while j >= 0i64 && nums[j as usize] > key {
            nums[(j + 1) as usize] = nums[j as usize];
            j -= 1;
        }
        nums[(j + 1) as usize] = key;
        for (i, num) in nums.iter().enumerate() {
            print!("{}{}", num, if i == nums.len()-1 {'\n'} else {' '}); 
        }
    }
}
