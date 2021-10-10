fn insertion_sort(a: &mut Vec<usize>, n: usize, g: isize, cnt: &mut usize) {
    for i in (g as usize)..n {
        let v = a[i as usize];
        let mut j: isize = (i as isize) - g;
        while j >= 0 && a[j as usize] > v {
            a[(j+g) as usize] = a[j as usize];
            j -= g;
            *cnt += 1;
        }
        a[(j+g) as usize] = v;
    }
}

fn shell_sort(a: &mut Vec<usize>, n: usize) {
    let mut cnt = 0;
    let m: usize = format!("{:b}", n).len();
    let g: Vec<usize> = (0..m).rev().map(|i| 2usize.pow(i as u32)).collect();
    for i in 0..m {
        insertion_sort(a, n, g[i] as isize, &mut cnt);
    }
    println!("{}", m);
    println!("{}", g.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" "));
    println!("{}", cnt);
    for i in a {
        println!("{}", i);
    }
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n: usize = n.trim().parse::<usize>().unwrap();
    let mut a = vec![];
    for _ in 0..n {
        let mut b = String::new();
        std::io::stdin().read_line(&mut b).ok();
        let b: usize = b.trim().parse::<usize>().unwrap();
        a.push(b);
    }
    shell_sort(&mut a, n);
}