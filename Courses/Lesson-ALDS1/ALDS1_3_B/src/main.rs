
/// First version -- too slow to pass #10
// fn main() {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     let mut iter = input.split_whitespace();
//     let n = iter.next().unwrap().parse::<u64>().unwrap();
//     let q = iter.next().unwrap().parse::<u64>().unwrap();

//     let mut queue = Queue::new();
    
//     for _ in 0..n {
//         let mut input = String::new();
//         std::io::stdin().read_line(&mut input).unwrap();
//         let mut iter = input.split_whitespace();
//         let name = iter.next().unwrap().to_string();
//         let time = iter.next().unwrap().parse::<u64>().unwrap();
//         queue.push((name, time));
//     }

//     let mut finished = vec![];
//     let mut time_passed = 0;

//     while let Some(item) = queue.pop() {
//         if item.1 <= q {
//             time_passed += item.1;
//             finished.push((item.0, time_passed));
//         } else {
//             let remaining_time = item.1 - q;
//             time_passed += q;
//             queue.push((item.0, remaining_time));
//         }
//     }
    
//     for (name, time) in finished {
//         println!("{} {}", name, time);
//     }
// }

// struct Queue {
//     queue: Vec<(String, u64)>
// }

// impl Queue {
//     fn new() -> Queue {
//         Queue {
//             queue: vec![]
//         }
//     }

//     fn push(&mut self, item: (String, u64)) {
//         self.queue.push(item);
//     }  

//     fn pop(&mut self) -> Option<(String, u64)> {

//         if self.queue.is_empty() {
//             return None;
//         } else {
//             return Some(self.queue.remove(0));
//         }
//     }
// }

/// Second version - faster
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap().parse::<u64>().unwrap();
    let q = iter.next().unwrap().parse::<u64>().unwrap();

    let mut queue = Queue::new();
    let mut finished = vec![];
    let mut time_passed = 0;
    
    for _ in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let name = iter.next().unwrap().to_string();
        let time = iter.next().unwrap().parse::<u64>().unwrap();

        // screening through all items first to remove the task
        // which can be finished within q
        if time <= q {
            time_passed += time;
            finished.push((name, time_passed));
        } else {
            let remaining_time = time - q;
            time_passed += q;
            queue.push((name, remaining_time));
        }
    }


    while let Some(item) = queue.pop() {
        if item.1 <= q {
            time_passed += item.1;
            finished.push((item.0, time_passed));
        } else {
            let remaining_time = item.1 - q;
            time_passed += q;
            queue.push((item.0, remaining_time));
        }
    }
    
    for (name, time) in finished {
        println!("{} {}", name, time);
    }
}

struct Queue {
    queue: Vec<(String, u64)>
}

impl Queue {
    fn new() -> Queue {
        Queue {
            queue: vec![]
        }
    }

    fn push(&mut self, item: (String, u64)) {
        self.queue.push(item);
    }  

    fn pop(&mut self) -> Option<(String, u64)> {

        if self.queue.is_empty() {
            return None;
        } else {
            return Some(self.queue.remove(0));
        }
    }
}
