use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().len() == 0{
            break;
        }
       
        for c in input.trim().to_string().chars() {
            match c {
                t @ 'A'..='z' => {
                    let count = map.entry(t.to_ascii_lowercase() as u8).or_insert(0);
                    *count += 1;
                },
                _ => ()
            }
        }
    }

    for alpha in 'a' as u8..='z' as u8 {
        if let Some(c) = map.get(&alpha) {
            println!("{} : {}", alpha as char, c);
        } else {
            println!("{} : {}", alpha as char, '0');
        }
    }
}
