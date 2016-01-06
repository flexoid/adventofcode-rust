extern crate crypto;

use std::io;
use crypto::md5;
use crypto::digest::Digest;

const ZEROS_COUNT: usize = 6;

fn main() {
    let mut secret_key = String::new();
    io::stdin().read_line(&mut secret_key).unwrap();
    let secret_key = secret_key.trim().to_string();

    let mut md5 = md5::Md5::new();
    let mut num = 1;
    let zeroes_string = &std::iter::repeat("0").take(ZEROS_COUNT).collect::<String>();

    loop {
        let hash_input = secret_key.clone() + &num.to_string();

        md5.input_str(&hash_input);
        if &md5.result_str()[..ZEROS_COUNT] == zeroes_string {
            println!("Answer: {}", num);
            return;
        }

        num += 1;
        md5.reset()
    }
}
