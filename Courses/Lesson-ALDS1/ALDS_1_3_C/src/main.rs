
use std::collections::LinkedList;
fn main() {
    let mut list: LinkedList<Option<i64>> = LinkedList::new();
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i64>().unwrap();
    for _ in 0..n {
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();
        if command.trim() == "deleteFirst" {
            while list.front().unwrap().is_none() {
                list.pop_front();
            }
            list.pop_front();
            // println!("delete first {:?}", list);
        } else if command.trim() == "deleteLast" {
            while list.back().unwrap().is_none() {
                list.pop_back();
            }
            list.pop_back();
            // println!("delete list {:?}", list);
        } else if command.trim().contains("insert") {
            let mut iter = command.split_whitespace();
            let _ = iter.next().unwrap();
            let data = iter.next().unwrap().parse::<i64>().unwrap();
            list.push_front(Some(data));
            // println!("insert {:?}", list);
        } else {
            let mut iter = command.split_whitespace();
            let _ = iter.next().unwrap();
            let data = iter.next().unwrap().parse::<i64>().unwrap();
            for item in list.iter_mut() {
                if *item == Some(data) {
                    *item = None;
                    break;
                } else {
                    continue;
                }
            }
            // println!("delete {:?}", list);
        }
    }

    let list: Vec<i64> = list.iter().filter(|&item| item.is_some()).map(|item| item.unwrap()).collect();
    for (i, item) in list.iter().enumerate() {
        print!("{}{}", item, if i == list.len()-1 {'\n'} else {' '});
    }
}