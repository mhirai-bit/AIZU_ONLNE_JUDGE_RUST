fn main() {
    let mut num_of_input = String::new();
    std::io::stdin().read_line(&mut num_of_input).unwrap();
    
    let mut numbers = String::new();
    std::io::stdin().read_line(&mut numbers).unwrap();
    let mut numbers: Vec<i64> = numbers.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();
    let mut swap = 0;
    if numbers.len() == 1 {
        println!("{}", numbers[0]);
        println!("{}", swap);
    }else{
        for i in 0..numbers.len() {
            for j in (i..numbers.len()-1).rev() {
                if numbers[j] > numbers[j+1] {
                    let temp = numbers[j];
                    numbers[j] = numbers[j+1];
                    numbers[j+1] = temp;
                    swap += 1;
                }
            }
        }
        let len = numbers.len()-1;
        for (i, number) in numbers.iter().enumerate() {
            print!("{}{}", number, if i == len {'\n'} else {' '});
        }
        println!("{}", swap);
    }
}
