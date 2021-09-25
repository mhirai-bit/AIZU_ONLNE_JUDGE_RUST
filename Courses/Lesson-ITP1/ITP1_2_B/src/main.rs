use std::io::*;

struct Input {
    line: Vec<String>,
    position: usize,
}

impl Input {
    fn new() -> Input {
        Input {
            line: vec![],
            position: 0
        }
    }

    fn read_word(&mut self) -> &String {
        if self.position == self.line.len() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.line = input.split_whitespace().map(|c| c.to_string()).collect();
            let result = &self.line[self.position];
            self.position += 1;
            return result;         
        }
        let result = &self.line[self.position];
        self.position += 1;
        result
    }
}

fn main() {
    let mut input = Input::new();
    let a: i32 = input.read_word().parse().unwrap();
    let b: i32 = input.read_word().parse().unwrap();
    let c: i32 = input.read_word().parse().unwrap();

    if a < b && b < c {
        println!("Yes");
    } else {
        println!("No");
    }
}



