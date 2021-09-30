fn main() {
    let mut cnt = 0;
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    loop {
        let mut sentence = String::new();
        std::io::stdin().read_line(&mut sentence).unwrap();
        for w in sentence.split_ascii_whitespace() {
            if w == "END_OF_TEXT" {
                println!("{}", cnt);
                std::process::exit(0);
            }
            if word.trim() == w.to_ascii_lowercase() {
                cnt += 1;
            }
        }
    }
}
