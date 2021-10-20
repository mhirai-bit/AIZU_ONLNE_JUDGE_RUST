fn main() {
    let mut n = String::new();
    let mut q = String::new();

    let mut A = String::new();
    let mut M = String::new();
    
    std::io::stdin().read_line(&mut n).unwrap();
    std::io::stdin().read_line(&mut A).unwrap();
    std::io::stdin().read_line(&mut q).unwrap();
    std::io::stdin().read_line(&mut M).unwrap();

    let n: i64 = n.trim().parse().unwrap();
    let q: i64 = q.trim().parse().unwrap();

    let A: Vec<i64> = A.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();
    let M: Vec<i64> = M.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();

    for m in M {
        if solve(0, m, n, &A) {
            println!("yes");
        } else {
            println!("no");
        }
    }
}



fn solve(i: i64, m: i64, n: i64, A: &[i64]) -> bool {
    if m == 0 {
        // ピッタリ足しきれる
        return true;
    } else if i >= n {
        // インデックスが超過
        return false;
    }
    //左項は、mを変えずにindex(i)を加算する=>A[i]を加算に使用しない
    //右項は、mからA[i]を引く=>A[i]を加算に使用する
    // iは0から始まってn-1まで行くので、mに対してAを網羅できる
    return solve(i + 1, m, n, A) || solve(i + 1, m - A[i as usize], n, A);
}

