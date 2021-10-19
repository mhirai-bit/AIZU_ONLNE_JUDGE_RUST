fn main() {
    let mut n = String::new();
    let mut input1 = String::new();
    let mut q = String::new();
    let mut input2 = String::new();

    std::io::stdin().read_line(&mut n).unwrap();
    std::io::stdin().read_line(&mut input1).unwrap();
    std::io::stdin().read_line(&mut q).unwrap();
    std::io::stdin().read_line(&mut input2).unwrap();
        
    let mut input1: Vec<i64> = input1.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();
    input1.sort();
    input1.dedup();
    let mut input2: Vec<i64> = input2.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();

    let mut count = 0;

    input2.sort();
    for num1 in input1 {
        let mut left = 0;
        let mut right = input2.len();
        while left < right {
            let mid = (left + right)/2;
            if num1 == input2[mid] {
                count += 1;
                break;
            } else if num1 < input2[mid] {
                // println!("index/2 = {}", index/2);
                right = mid;
            } else {
                left = mid + 1;
            }
        }
    }
    println!("{}", count);
}
