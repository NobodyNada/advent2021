use std::io::{self, BufRead};

pub fn run() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| {
            line.expect("read error")
                .parse::<u32>()
                .expect("parse error")
        })
        .collect::<Vec<_>>();

    println!("{}", input.windows(2).filter(|w| w[0] < w[1]).count());
}
