//Print a Frame

fn main() {
    let mut rectangles: Vec<Rectangle> = vec![];
    loop{
        match read() {
            (0, 0) => break,
            (a, b) => {
                let rectangle = Rectangle::new((a, b));
                rectangles.push(rectangle);
            }
        }
    }
    for rectangle in rectangles {
        rectangle.draw();
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

struct Rectangle {
    h: u32,
    w: u32
}

impl Rectangle {
    fn new((h, w): (u32, u32)) -> Rectangle {
        Rectangle {h, w}
    }

    fn draw(&self) {
        for i in 1..=self.h {
            for j in 1..=self.w {
                if i == 1 || i == self.h || j == 1 || j == self.w {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}