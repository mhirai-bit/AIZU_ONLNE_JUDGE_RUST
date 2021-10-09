

fn main() {
    let mut num_of_input = String::new();
    std::io::stdin().read_line(&mut num_of_input).unwrap();
    let mut cards = String::new();
    std::io::stdin().read_line(&mut cards).unwrap();
    let cards: Vec<String> = cards.split_whitespace().map(|c| c.to_string()).collect();
    let mut cards_vec = vec![];
    for card in cards {
        let (suit, num) = card.split_at(1);
        // println!("suit = {}, num = {}", suit, num);
        let suit = suit.to_owned();
        let num = num.trim().to_owned();
        let num = num.trim().parse::<u8>().unwrap();
        cards_vec.push((suit, num));
    }
    let selection = selection_sort(&cards_vec);
    let bubble = bubble_sort(cards_vec);

    let bubble_stable = true;
    let mut selection_stable = true;
    for (s, b) in selection.iter().zip(bubble.iter()) {
        let (s_suit, s_num) = s.clone();
        let (b_suit, b_num) = b.clone();
        if s_suit !=b_suit || s_num != b_num {
            selection_stable = false;
            break;
        }  
    }

    print(&bubble, bubble_stable);
    print(&selection, selection_stable);
}


fn print(cards_vec: &Vec<(String, u8)>, stable: bool) {
    let len = cards_vec.len()-1;
    for (i, (suit, num)) in cards_vec.iter().enumerate() {
        print!("{}{}{}", suit, num, if i == len {'\n'} else {' '});
    }
    if stable == true {
        println!("{}", "Stable");
    } else {
        println!("{}", "Not stable");
    }
}

fn bubble_sort(mut cards_vec: Vec<(String, u8)>) -> Vec<(String, u8)>{
    let mut suits = vec![];
    let mut nums = vec![];
    for (suit, num) in cards_vec{
        suits.push(suit);
        nums.push(num);
    }
    // let mut nums_clone = nums.clone();
    // nums_clone.sort();
    // let mut matching = true;
    // for (a, b) in nums.iter().zip(nums_clone) {
    //     if *a != b {
    //         matching = false;
    //         break;
    //     }
    // }
    // if matching {
    //      stable = true;      
    // }
    for i in 0..nums.len() {
        for j in (i+1..nums.len()).rev() {
            if nums[j-1] > nums[j] {
                let temp = suits[j-1].clone();
                suits[j-1] = suits[j].clone();
                suits[j] = temp;
                let temp = nums[j-1].clone();
                nums[j-1] = nums[j].clone();
                nums[j] = temp;
            }
        }
    }
    let mut cards_vec = vec![];
    for (suit, num) in suits.iter().zip(&nums).map(|(a, b)| (a, *b)){
        cards_vec.push((suit.clone(), num));
    }
    cards_vec
}

fn selection_sort(cards_vec: &Vec<(String, u8)>) -> Vec<(String, u8)> {
    let mut suits = vec![];
    let mut nums = vec![];
    for (suit, num) in cards_vec{
        suits.push(suit);
        nums.push(num);
    }
    for i in 0..nums.len() {
        let min = nums[i..].iter().min().unwrap();
        let index = nums[i..].iter().position(|&r| r == *min).unwrap();
        if *min < nums[i] {
            let temp = nums[i];
            nums[i] = nums[i..][index];
            nums[i..][index] = temp;
            let temp = suits[i];
            suits[i] = suits[i..][index];
            suits[i..][index] = temp;
        }
    }
    let mut cards_vec = vec![];
    for (suit, num) in suits.iter().zip(&nums).map(|(&a, &b)| (a, b)){
        cards_vec.push((suit.clone(), *num));
    }
    cards_vec
}