fn main() {
    let mut swap = 0;
    let mut num_of_input = String::new();
    std::io::stdin().read_line(&mut num_of_input).unwrap();
    let mut numbers = String::new();
    std::io::stdin().read_line(&mut numbers).unwrap();
    let mut numbers: Vec<i64> = numbers.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();
    if numbers.len() == 1 {
        println!("{}", numbers[0]);
        println!("{}", swap);
    } else {
        for i in 0..numbers.len() {
            let min = *numbers[i..].iter().min().unwrap();
            let index = numbers[i..].iter().position(|&r| r==min).unwrap();
            // println!("min = {}, index = {}", min, index);
            if min < numbers[i] {
                let temp = numbers[i];
                numbers[i] = min;
                numbers[i..][index] = temp;
                swap += 1;
            }
            let len = numbers.len()-1;
            // for (i, number) in numbers.iter().enumerate(){
            //     print!("{}{}", number, if i == len {'\n'} else {' '});
            // }
        }
        let len = numbers.len()-1;
        for (i, number) in numbers.iter().enumerate(){
            print!("{}{}", number, if i == len {'\n'} else {' '});
        }
        println!("{}", swap);
    }
}
