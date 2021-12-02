use std::io::{self, BufRead};

pub fn run() {
    let stdin = io::stdin();
    let (x, y, _) = stdin
        .lock()
        .lines()
        .map(|line| {
            let line = line.expect("read error");
            let (dir, num) = line.split_once(" ").expect("parse error");
            (dir.to_owned(), num.parse::<i64>().expect("parse error"))
        })
        .fold((0i64, 0i64, 0i64), |(x, y, aim), (dir, num)| {
            match dir.as_str() {
                "forward" => (x + num, y + num * aim, aim),
                "back" => (x - num, y - num * aim, aim),
                "down" => (x, y, aim + num),
                "up" => (x, y, aim - num),
                _ => panic!("invalid direction: {}", dir),
            }
        });
    println!("{}", x * y);
}
