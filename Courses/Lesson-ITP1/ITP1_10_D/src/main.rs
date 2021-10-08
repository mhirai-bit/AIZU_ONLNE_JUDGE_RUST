fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();

    let x = input();
    let y = input();

    let mut ans1: f64 = 0.0;
    for i in 0..n {
        ans1 += (x[i] - y[i]).abs();
    }

    let mut ans2: f64 = 0.0;
    for i in 0..n {
        ans2 += (x[i] - y[i]).powf(2.0);
    }
    ans2 = ans2.sqrt();

    let mut ans3: f64 = 0.0;
    for i in 0..n {
        ans3 += (x[i] - y[i]).abs().powf(3.0);
    }
    ans3 = ans3.cbrt();

    let mut ans4: f64 = 0.0;
    for i in 0..n {
        ans4 = if (x[i] - y[i]).abs() > ans4 {
            (x[i] - y[i]).abs()
        } else {
            ans4
        };
    }

    println!("{}\n{}\n{}\n{}", ans1, ans2, ans3, ans4);
}

fn input() -> Vec<f64> {
   let mut s = String::new();
   std::io::stdin().read_line(&mut s).unwrap();

   let v: Vec<f64> = s.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<_>();

   v
}

