use std::io::{self, BufRead};

pub fn run() {
    let stdin = io::stdin();
    let (x, y) = stdin
        .lock()
        .lines()
        .map(|line| {
            let line = line.expect("read error");
            let (dir, num) = line.split_once(" ").expect("parse error");
            (dir.to_owned(), num.parse::<u32>().expect("parse error"))
        })
        .fold((0, 0), |(x, y), (dir, num)| match dir.as_str() {
            "forward" => (x + num, y),
            "back" => (x - num, y),
            "down" => (x, y + num),
            "up" => (x, y - num),
            _ => panic!("invalid direction: {}", dir),
        });
    println!("{}", x * y);
}
