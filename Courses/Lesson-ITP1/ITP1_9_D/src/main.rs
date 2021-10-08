use std::io;

struct MyString {
    string: String,
}

impl MyString {
    fn new(string: String) -> MyString {
        MyString {
            string,
        }
    }

    fn replace(&mut self, a: usize, b: usize, string_to_replace: &str) {
        let left = self.string.get(..a).unwrap();
        let right = self.string.get(b..).unwrap();
        self.string = left.to_string() + string_to_replace + right;
    }

    fn reverse(&mut self, a: usize, b: usize) {
        let rev_string = self.string.get(a..b).unwrap().chars().rev().collect::<String>();
        self.string = self.string.get(..a).unwrap().to_string() + &rev_string + self.string.get(b..).unwrap()
    }

    fn print(&self, a: usize, b: usize){
        println!("{}", self.string.get(a..b).unwrap());        
    }
}

fn read(s: &mut String){
    io::stdin().read_line(s).unwrap();
}

fn main() {
    let mut string = String::new();
    read(&mut string);
    let mut my_string = MyString::new(string);
    let mut num = String::new();
    read(&mut num);
    let num = num.trim().parse::<u8>().unwrap();
    
    for _ in 0..num {
        let mut command = String::new();
        read(&mut command);
        let mut iter = command.split_whitespace();
        let command = iter.next();
        let a = iter.next().unwrap().trim().parse::<usize>().unwrap();
        let b = iter.next().unwrap().trim().parse::<usize>().unwrap();
        match command {
            Some(c) => match c {
                "replace" => {
                    let p = iter.next().unwrap().trim().parse::<String>().unwrap();
                    my_string.replace(a, b+1, &p); 
                },
                "reverse" => {
                    my_string.reverse(a, b+1);
                },
                "print" => {
                    my_string.print(a, b+1);
                },
                _ => ()
            },
            _ => ()
        }
    }

}
