#[derive(Debug)]
struct Dice {
    top: usize,
    front: usize,
    right: usize,
    numbers: Vec<usize>,
}

impl Dice {
    fn new(numbers: Vec<usize>) -> Dice {
        Dice {
            top: 0,
            front: 1,
            right: 2,
            numbers: numbers,
        }
    }

    fn get_right_idx(&self, top: usize, front: usize) -> usize {
        if top == 0 {
            match front {
                1 => { return 2; },
                2 => { return 4; },
                3 => { return 1; },
                4 => { return 3; },
                _ => { return 6; }
            }
        } else if top == 1 {
            match front {
                0 => { return 3; },
                2 => { return 0; },
                3 => { return 5; },
                5 => { return 2; },
                _ => { return 6; }
            }
        } else if top == 2 {
            match front {
                0 => { return 1; },
                1 => { return 5; },
                4 => { return 0; },
                5 => { return 4; },
                _ => { return 6; }
            }
        } else if top == 3 {
            match front {
                0 => { return 4; },
                1 => { return 0; },
                4 => { return 5; },
                5 => { return 1; },
                _ => { return 6; }
            }
        } else if top == 4 {
            match front {
                0 => { return 2; },
                2 => { return 5; },
                3 => { return 0; },
                5 => { return 3; },
                _ => { return 6; }
            }
        } else if top == 5 {
            match front {
                1 => { return 3; },
                2 => { return 1; },
                3 => { return 4; },
                4 => { return 2; },
                _ => { return 6; }
            }
        }
        6
    }
}


fn main() {
    let mut labels = String::new();
    std::io::stdin().read_line(&mut labels).ok();
    let labels: Vec<usize> = labels.trim().split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n: usize = n.trim().parse::<usize>().unwrap();
    let dice = Dice::new(labels);
    for _ in 0..n {
        let mut tf = String::new();
        std::io::stdin().read_line(&mut tf).ok();
        let tf: Vec<usize> = tf.trim().split_whitespace().map(|c| c.parse::<usize>().unwrap()).collect();
        let top_idx = dice.numbers.iter().enumerate().find(|pair| *pair.1 == tf[0]).unwrap().0;
        let front_idx = dice.numbers.iter().enumerate().find(|pair| *pair.1 == tf[1]).unwrap().0;
        println!("{}", dice.numbers[dice.get_right_idx(top_idx, front_idx)]);
    }
}
