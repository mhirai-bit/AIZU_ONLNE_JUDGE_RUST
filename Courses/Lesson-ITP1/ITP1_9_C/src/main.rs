fn main() {
    let mut taro_score = 0;
    let mut hanako_score = 0;
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).unwrap();
    let num = num.trim().parse::<u16>().unwrap();
    for _ in 0..num {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_ascii_whitespace();
        let taro: String = iter.next().unwrap().to_string();
        let hanako: String = iter.next().unwrap().to_string();
        if taro > hanako {
            taro_score += 3;
        } else if hanako > taro {
            hanako_score += 3;
        } else {
            taro_score += 1;
            hanako_score += 1;
        }
    }
    println!("{} {}", taro_score, hanako_score);
}
