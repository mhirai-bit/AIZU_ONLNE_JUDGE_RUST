use std::collections::HashSet;

fn main() {

    let mut hash = HashSet::new();
    let mut ans = vec![];

    let mut n = String::new();
    
    std::io::stdin().read_line(&mut n).unwrap();
    let n: u64 = n.trim().parse().unwrap();
    
    for _ in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let command  = iter.next().unwrap();
        let str = iter.next().unwrap();
        match command {
            "insert" => {
                hash.insert(str.to_string());
            },
            "find" => {
                if hash.contains(str) {
                    ans.push("yes");
                } else {
                    ans.push("no");
                }
            },
            _ => ()
        }
    }   
    for a in ans {
        println!("{}", a);
    }
}
