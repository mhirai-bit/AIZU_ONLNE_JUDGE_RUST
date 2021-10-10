fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut stack = Stack::new();
    for item in input.split_whitespace() {
        match item {
            " "  => (),
            "+" => {
                let a = stack.pop();
                let b = stack.pop();
                let ans = add(a, b);
                stack.push(ans.to_string());
            },
            "-" => {
                let a = stack.pop();
                let b = stack.pop();
                let ans = subtract(a, b);
                stack.push(ans.to_string());
            },
            "*" => {
                let a = stack.pop();
                let b = stack.pop();
                let ans = multiply(a, b);
                stack.push(ans.to_string());
            },
            "/" => {
                let a = stack.pop();
                let b = stack.pop();
                let ans = devide(a, b);
                stack.push(ans.to_string());
            },
            _ => {
                stack.push(item.to_string());
            }
        }
    }
    println!("{}", stack.pop());
}

struct Stack {
    stack: Vec<String>
}

impl Stack {
    fn new() -> Stack {
        Stack {
            stack : vec![]
        }
    }

    fn push(&mut self, item: String) {
        self.stack.push(item);
    }

    fn pop(&mut self) -> String {
        match self.stack.pop() {
            Some(item) => return item,
            None => return String::from("stack is empty")
        }
    }
}


fn add(a: String, b: String) -> i64 {
    let a = a.trim().parse::<i64>().unwrap();
    let b = b.trim().parse::<i64>().unwrap();
    // println!("add is called with {} + {} = {}", a, b, a+b);
    a + b
}
fn subtract(a: String, b: String) -> i64 {
    let a = a.trim().parse::<i64>().unwrap();
    let b = b.trim().parse::<i64>().unwrap();
    // println!("subtract is called with {} - {} = {}", a, b, b-a);
    b - a
}
fn multiply(a: String, b: String) -> i64 {
    let a = a.trim().parse::<i64>().unwrap();
    let b = b.trim().parse::<i64>().unwrap();
    // println!("multiply is called with {} * {} = {}", a, b, a*b);
    a * b
}
fn devide(a: String, b: String) -> i64 {
    let a = a.trim().parse::<i64>().unwrap();
    let b = b.trim().parse::<i64>().unwrap();
    b / a
}