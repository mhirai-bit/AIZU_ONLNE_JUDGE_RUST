const SENTINEL: i64 = 10000000000;
static mut COMPALISON: u64 = 0; 

fn main() {
    let mut n = String::new();
    let mut input = String::new();

    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut A: Vec<i64> = input.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();
    merge_sort(&mut A, 0, n);
    for (index, num) in A.iter().enumerate() {
        print!("{}{}", num, if index == A.len()-1 {'\n'} else {' '});
    }
    unsafe {
        println!("{}", COMPALISON);
    }
}

fn merge(A: &mut Vec<i64>, left: usize, mid: usize, right: usize) {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut L: Vec<i64> = vec![0; n1+1];
    let mut R: Vec<i64> = vec![0; n2+1];
    for i in 0..n1 {
        L[i] = A[left + i];
    }
    for i in 0..n2 {
        R[i] = A[mid + i];
    }
    L[n1] = SENTINEL;
    R[n2] = SENTINEL;
    let mut i = 0;
    let mut j = 0;
    for k in left..right {
        unsafe {
            COMPALISON += 1;
        }
        if L[i] <= R[j] {
            A[k] = L[i];
            i += 1;
        } else {
            A[k] = R[j];
            j += 1;
        }
    }
}


fn merge_sort(A: &mut Vec<i64>, left: usize, right: usize) {
    if left + 1usize < right {
        let mid = (left + right)/2;
        merge_sort(A, left, mid);
        merge_sort(A, mid, right);
        merge(A, left, mid, right);
    }
}
