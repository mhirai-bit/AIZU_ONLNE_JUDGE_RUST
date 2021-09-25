use std::io::*;

struct Input {
    line : Vec<String>,
    position : usize
}

impl Input {
    fn new() -> Input {
        Input {
            line : vec![],
            position : 0,
        }
    }

    fn read_word(&mut self) -> &String {
        if self.line.len() == self.position {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok().unwrap();
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
    let a = input.read_word().parse::<i32>().unwrap();
    let b = input.read_word().parse::<i32>().unwrap();
    if a > b {
        println!("a > b");
    } else if a < b {
        println!("a < b");
    } else {
        println!("a == b");
    }
}