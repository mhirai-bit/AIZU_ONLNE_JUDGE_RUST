fn main() {
    loop {
        let mut results = vec![];
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: u32 = iter.next().unwrap().parse().unwrap();
        let x: u32 = iter.next().unwrap().parse().unwrap();
        let v: Vec<u32> = (1..=n).collect();
        if n == 0 && x == 0 {break;}
        for i in 0..v.len() {
            for j in i+1..v.len(){
                for k in j+1..v.len() {
                    if v[i] + v[j] + v[k] == x{
                        results.push((i+1, j+1, k+1));
                    }
                }
            }
        }
        println!("{}", results.len());
    }
}
