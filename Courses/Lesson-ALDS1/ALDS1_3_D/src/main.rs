fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut index_stack: Vec<usize> = vec![];
    let mut area_stack: Vec<(usize, usize)> = vec![];
    let mut area;
    
    for (current_index, c) in input.chars().enumerate(){
        match c {
            '\\' => {
                index_stack.push(current_index);
            },
            '/' => {
                if index_stack.len() != 0 {
                    let popped_index = index_stack.pop().unwrap();
                    area = current_index - popped_index;
                    area_stack.push((popped_index, area));

                    while area_stack.len() >= 2 && area_stack[area_stack.len()-1].0 < area_stack[area_stack.len()-2].0 { 
                        let (index, area1) = area_stack.pop().unwrap(); 
                        let (_, area2) = area_stack.pop().unwrap(); 
                        let merged_area = area1 + area2;
                        area_stack.push((index, merged_area));
                    }
                }
            },
            _ => ()
        }
    }

    let total_area = area_stack.iter().fold(0,|acc, tuple| acc + tuple.1);
    println!("{}", total_area);
    print!("{}{}", area_stack.len(), if area_stack.len() == 0 {'\n'} else {' '});

    for (i, area_tuple) in area_stack.iter().enumerate() {
        print!("{}{}", area_tuple.1, if i == area_stack.len()-1 {'\n'} else {' '});
    }
}
