fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let r: usize = iter.next().unwrap().parse().unwrap();
    let c: usize = iter.next().unwrap().parse().unwrap();
    let mut table = vec![];
    for _ in 0..r {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let mut row = vec![];
        let mut sum = 0;
        for _ in 0..c {
            let val = iter.next().unwrap().parse::<u32>().unwrap();
            sum += val;
            row.push(val);
        }
        row.push(sum);
        table.push(row);
    }
    let mut col_sum = vec![];
    for i in 0..=c {
        let mut sum = 0;
        for j in 0..r {
            sum += table[j][i];
        }
        col_sum.push(sum);
    }
    table.push(col_sum);
    for row in &table {
        let mut col_cnt = 0;
        for val in row {
            print!("{}", val);
            col_cnt += 1;
            if col_cnt < c + 1 {
                print!(" ");
            }
        }  
        println!();
    }
}
