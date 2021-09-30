fn main() {
    let mut result = vec![];
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input == "0" {
            break;
        }
        let mut sum = 0;
        for c in input.chars() {
            sum += c.to_digit(10).unwrap();
        }
        result.push(sum);
    }

    for sum in result {
        println!("{}", sum);
    }
}
