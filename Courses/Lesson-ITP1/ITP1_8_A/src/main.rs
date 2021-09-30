fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    for c in input.chars() {
        let mut converted: char = ' ';
        if c.is_ascii_lowercase() {
            converted = c.to_ascii_uppercase();
        } else if c.is_ascii_uppercase() {
            converted = c.to_ascii_lowercase();
        } else {
            converted = c;
        }
        print!("{}", converted);
    }
}
