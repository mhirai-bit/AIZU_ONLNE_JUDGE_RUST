//Print a Chessboard

fn main() {
    let mut chessboards: Vec<Chessboard> = vec![];
    loop{
        match read() {
            (0, 0) => break,
            (a, b) => {
                let chessboard = Chessboard::new((a, b));
                chessboards.push(chessboard);
            }
        }
    }
    for chessboard in chessboards {
        chessboard.draw();
    }
}

fn read() -> (u32, u32) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let w = iter.next().unwrap().parse().unwrap();
    let h = iter.next().unwrap().parse().unwrap();
    (w, h)
}

struct Chessboard {
    h: u32,
    w: u32
}

impl Chessboard {
    fn new((h, w): (u32, u32)) -> Chessboard {
        Chessboard {h, w}
    }

    fn draw(&self) {
        for i in 1..=self.h {
            for j in 1..=self.w {
                if i % 2 == 0 && j % 2 == 1 {
                    print!(".");
                } else if i % 2 == 1 && j % 2 == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
        println!();
    }
}