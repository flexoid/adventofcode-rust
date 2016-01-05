use std::io;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            let floor = calc_floor(&buffer);
            println!("Floor: {}", floor);
        },
        Err(error) => panic!(error),
    }
}

fn calc_floor(buffer: &str) -> i32 {
    buffer.chars().fold(0, |acc, ch| {
        acc + match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    })
}
