fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let l: usize = iter.next().unwrap().parse().unwrap();
    
    let mut A = vec![vec![0; m]; n];
    let mut B = vec![vec![0; l]; m];
    
    read_matrix(&mut A, n, m);
    read_matrix(&mut B, m, l);
    let C = multiply_matrices(A, B);
    print_matrices(C);
}

fn read_matrix(matrix: &mut Vec<Vec<u64>>, row: usize, column: usize) {
    for i in 0..row {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        for j in 0..column {
            matrix[i][j] = iter.next().unwrap().parse::<u64>().unwrap();
        }
    }
}

fn multiply_matrices(A: Vec<Vec<u64>>, B: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut C: Vec<Vec<u64>> = vec![vec![0; B[0].len()]; A.len()];
    for a_row in 0..A.len() {
        for b_row in 0..B.len() {
            for b_col in 0..B[0].len() {
                C[a_row][b_col] += A[a_row][b_row] * B[b_row][b_col]
            }
        }
    }
    //3  3       3  2      2 3
    // C[0][0] += A[0][0] * B[0][0] 
    // C[0][0] += A[0][1] * B[1][0]
    // C[0][1] += A[0][0] * B[0][1] 
    // C[0][1] += A[0][1] * B[1][1]
    // C[0][2] += A[0][0] * B[0][2] 
    // C[0][2] += A[0][1] * B[1][2]
    
    // C[1][0] += A[1][0] * B[0][0] 
    // C[1][0] += A[1][1] * B[1][0]
    // C[1][1] += A[1][0] * B[0][1] 
    // C[1][1] += A[1][1] * B[1][1]
    // C[1][2] += A[1][0] * B[0][2] 
    // C[1][2] += A[1][1] * B[1][2]
    
    // C[2][0] += A[2][0] * B[0][0] 
    // C[2][0] += A[2][1] * B[1][0]
    // C[2][1] += A[2][0] * B[0][1] 
    // C[2][1] += A[2][1] * B[1][1]
    // C[2][2] += A[2][0] * B[0][2] 
    // C[2][2] += A[2][1] * B[1][2]
    C
}

fn print_matrices(C: Vec<Vec<u64>>) {
    for row in C {
        for (j, item) in row.iter().enumerate() {
            print!("{}{}", item, if j != row.len()-1 {' '} else {'\n'});
        }
    }
}

