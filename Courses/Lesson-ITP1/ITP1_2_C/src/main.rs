use std::io::*;

struct Input {
    line: Vec<String>,
    position: usize
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
    let a: u32 = input.read_word().parse().unwrap();
    let b: u32 = input.read_word().parse().unwrap();
    let c: u32 = input.read_word().parse().unwrap();

    let mut v = vec![a, b, c];
    v.sort();

    println!("{} {} {}", v[0], v[1], v[2]);
}
