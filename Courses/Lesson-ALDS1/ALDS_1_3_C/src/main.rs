use std::ops::Deref;

fn main() {
    println!("Hello, world!");
}


struct Item {
    key: u64,
    prev: Option<Box<Item>>,
    next: Option<Box<Item>>
}

