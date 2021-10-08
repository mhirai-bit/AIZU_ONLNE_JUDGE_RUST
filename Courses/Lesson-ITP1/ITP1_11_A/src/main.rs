// use std::collections::HashMap;

// #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// enum Top {
//     One,
//     Two,
//     Three,
//     Four,
//     Five,
//     Six
// }


// struct Dice {
//     pub top: Option<Top>
// }

// impl Dice {
//     fn new() -> Dice {
//         Dice {
//             top: Some(Top::One)
//         }
//     }

//     fn roll(&mut self, direction: char) {
//         self.top = match self.top.unwrap() {
//             Top::One => {
//                 match direction {
//                     'N' => {
//                         Some(Top::Two)
//                     },
//                     'E' => {
//                         Some(Top::Four)
//                     },
//                     'S' => {
//                         Some(Top::Five)
//                     },
//                     'W' => {
//                         Some(Top::Four)
//                     },
//                     _ => None
//                 }
//             },
//             Top::Two => {
//                 match direction {
//                     'N' => {
//                         Some(Top::Six)
//                     },
//                     'E' => {
//                         Some(Top::Four)
//                     },
//                     'S' => {
//                         Some(Top::One)
//                     },
//                     'W' => {
//                         Some(Top::Three)
//                     },
//                     _ => None
//                 }
//             },
//             Top::Three => {
//                 match direction {
//                     'N' => {
//                         Some(Top::Two)
//                     },
//                     'E' => {
//                         Some(Top::One)
//                     },
//                     'S' => {
//                         Some(Top::Five)
//                     },
//                     'W' => {
//                         Some(Top::Six)
//                     },
//                     _ => None
//                 }
//             },
//             Top::Four => {
//                 match direction {
//                     'N' => {
//                         Some(Top::Two)
//                     },
//                     'E' => {
//                         Some(Top::Six)
//                     },
//                     'S' => {
//                         Some(Top::Five)
//                     },
//                     'W' => {
//                         Some(Top::One)
//                     },
//                     _ => None
//                 }
//             },
//             Top::Five => {
//                 match direction {
//                     'N' => {
//                         Some(Top::One)
//                     },
//                     'E' => {
//                         Some(Top::Four)
//                     },
//                     'S' => {
//                         Some(Top::Six)
//                     },
//                     'W' => {
//                         Some(Top::Three)
//                     },
//                     _ => None
//                 }
//             },
//             Top::Six => {
//                 match direction {
//                     'N' => {
//                         Some(Top::Five)
//                     },
//                     'E' => {
//                         Some(Top::Four)
//                     },
//                     'S' => {
//                         Some(Top::Two)
//                     },
//                     'W' => {
//                         Some(Top::Three)
//                     },
//                     _ => None
//                 }
//             }
//         };
//         println!("self.top = {:?}", self.top);
//     }
// }


// fn main() {
//     let mut dice_map = HashMap::new();
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     let mut iter = input.split_whitespace();
//     dice_map.insert(Top::One, iter.next().unwrap().parse::<usize>().unwrap());
//     dice_map.insert(Top::Two, iter.next().unwrap().parse::<usize>().unwrap());
//     dice_map.insert(Top::Three, iter.next().unwrap().parse::<usize>().unwrap());
//     dice_map.insert(Top::Four, iter.next().unwrap().parse::<usize>().unwrap());
//     dice_map.insert(Top::Five, iter.next().unwrap().parse::<usize>().unwrap());
//     dice_map.insert(Top::Six, iter.next().unwrap().parse::<usize>().unwrap());
//     let mut dice = Dice::new();
//     let mut instruction = String::new();
//     std::io::stdin().read_line(&mut instruction).unwrap();
//     for c in instruction.trim().chars() {
//         dice.roll(c);
//     }
//     println!("{}", dice_map.get(&dice.top.unwrap()).unwrap());
// }


#[derive(Debug)]
struct Dice {
    top: usize,
    front: usize,
    right: usize,
    numbers: Vec<isize>,
}

impl Dice {
    fn new(numbers: Vec<isize>) -> Dice {
        Dice { top: 0, front: 1, right: 2, numbers: numbers }
    }

    fn moves(&mut self, direction: char) {
        match direction {
            'S' => {
                let temp = self.front;
                self.front = self.top;
                self.top = 5 - temp;
            },
            'N' => {
                let temp = self.top;
                self.top = self.front;
                self.front = 5 - temp;
            },
            'W' => {
                let temp = self.top;
                self.top = self.right;
                self.right = 5 - temp;
            },
            'E' => {
                let temp = self.top;
                self.top = 5 - self.right;
                self.right = temp;
            },
            _ => {}
        }
    }

    fn get_top_val(&mut self) -> isize {
        self.numbers[self.top]
    }
}

fn main() {
    let mut labels = String::new();
    std::io::stdin().read_line(&mut labels).ok();
    let labels: Vec<isize> = labels.trim().split_whitespace()
                    .map(|c| c.parse::<isize>().unwrap())
                    .collect();
    let mut moves = String::new();
    std::io::stdin().read_line(&mut moves).ok();
    let moves: &str = moves.trim();
    let mut dice = Dice::new(labels);
    for m in moves.chars() {
        dice.moves(m);
    }
    println!("{}", dice.get_top_val());

}