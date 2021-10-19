use std::collections::HashSet;

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let mut nums1 = String::new();
    std::io::stdin().read_line(&mut nums1).unwrap();
    let mut q = String::new();
    std::io::stdin().read_line(&mut q).unwrap();
    let mut nums2= String::new();
    std::io::stdin().read_line(&mut nums2).unwrap();

    let a: HashSet<i64> = nums1.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();
    let b: HashSet<i64> = nums2.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();

    let c: HashSet<_> = a.intersection(&b).collect();
    println!("{}", c.len());
}
