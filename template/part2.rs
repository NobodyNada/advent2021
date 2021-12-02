use std::io::{self, BufRead};

pub fn run() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        println!("{}", line.expect("read error"));
    }
    todo!()
}
