fn main() {
    let mut grades = vec![];

    let mut input = String::new();

    loop {
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let m: i8 = iter.next().unwrap().parse().unwrap();
        let f: i8 = iter.next().unwrap().parse().unwrap();
        let r: i8 = iter.next().unwrap().parse().unwrap();
        match (m, f, r) {
            (-1, -1, -1) => break,
            (-1, _, _) | (_, -1, _) => grades.push("F"),
            _ => {
                let total = m + f;
                if total >= 80 {
                    grades.push("A");
                } else if 80 > total && total >= 65 {
                    grades.push("B");
                } else if 65 > total && total >= 50 {
                    grades.push("C");
                } else if 50 > total && total >= 30 {
                    if r >= 50 {
                        grades.push("C");
                    } else {
                        grades.push("D");
                    }
                } else if 30 > total {
                    grades.push("F")
                }
            }
        }
        input.clear(); 
    }
    for grade in grades {
        println!("{}", grade);
    }
}
