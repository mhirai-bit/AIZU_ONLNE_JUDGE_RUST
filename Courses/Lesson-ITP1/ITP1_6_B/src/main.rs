//Finding Missing Cards

struct Cards {
    spades: Vec<u8>,
    hearts: Vec<u8>,
    clubs: Vec<u8>,
    diamonds: Vec<u8>
}

impl Cards {
    fn new() -> Cards {
        let mut cards = Cards{
            spades: vec![],
            hearts: vec![],
            clubs: vec![],
            diamonds: vec![]
        };
        for i in 1..=13 {
            cards.spades.push(i);
            cards.hearts.push(i);
            cards.clubs.push(i);
            cards.diamonds.push(i);
        }
        cards
    }

    fn check(&mut self, (suits, num): (&str, u8)) {
        match (suits, num) {
            ("S", num) => {
                let index = self.spades.iter().position(|x| *x == num).unwrap();
                self.spades.remove(index);           
            },
            ("H", num) => {
                let index = self.hearts.iter().position(|x| *x == num).unwrap();
                self.hearts.remove(index);           
            },
            ("C", num) => {
                let index = self.clubs.iter().position(|x| *x == num).unwrap();
                self.clubs.remove(index);           
            },
            ("D", num) => {
                let index = self.diamonds.iter().position(|x| *x == num).unwrap();
                self.diamonds.remove(index);           
            }
            _ => ()
        }
    }

    fn print(&self) {
        for s in &self.spades {
            println!("{} {}", 'S', s);
        }
        for h in &self.hearts {
            println!("{} {}", 'H', h);
        }
        for c in &self.clubs {
            println!("{} {}", 'C', c);
        }
        for d in &self.diamonds {
            println!("{} {}", 'D', d);
        }
    }
}

fn main(){
    let mut cards = Cards::new();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: u8 = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let suit: &str = iter.next().unwrap();
        let num: u8 = iter.next().unwrap().parse().unwrap();
        cards.check((suit, num));
    }
    cards.print();
}