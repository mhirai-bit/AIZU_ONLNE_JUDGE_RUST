fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut pattern = String::new();
    std::io::stdin().read_line(&mut pattern).unwrap();
    if input.trim().repeat(2).contains(&pattern.trim()){
        println!("Yes");
    } else {
        println!("No");
    }
}
