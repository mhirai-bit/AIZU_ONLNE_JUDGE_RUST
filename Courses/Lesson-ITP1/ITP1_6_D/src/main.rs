fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let m = iter.next().unwrap().parse::<usize>().unwrap();
    let mut A = vec![vec![]; n];
    readA(&mut A, m);
    let mut b = vec![];
    readb(&mut b, m);
    let result = multiply(A, b);
    for val in result {
        println!("{}", val);
    }
}

fn readA(A: &mut Vec<Vec<i32>>, len: usize) {
    let mut input = String::new();
    for i in 0..A.len() {
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        for _ in 0..len {
            A[i].push(iter.next().unwrap().parse::<i32>().unwrap())
        }
        input.clear();
    }
}

fn readb(b: &mut Vec<i32>, len: usize) {
    let mut input = String::new();
    for _ in 0..len {
        std::io::stdin().read_line(&mut input).unwrap();
        b.push(input.trim().parse::<i32>().unwrap());
        input.clear();
    }
}

fn multiply(A: Vec<Vec<i32>>, b: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; A.len()];
    for i in 0..A.len() {
        for j in 0..b.len() {
            result[i] += A[i][j] * b[j];
        }
    }
    result
}