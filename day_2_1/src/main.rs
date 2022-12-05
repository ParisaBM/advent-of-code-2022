use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut score = 0;
    for line in lines {
        let line = line.unwrap();
        let line = line.as_bytes();

        // We can represent the throws as rock = 0, paper = 1, and scissors = 2
        let opponent_throw = line[0] - 'A' as u8;
        let self_throw = line[2] - 'X' as u8;
        // Taking a difference %3 can give us the result of a game
        // Adding 4 instead of 1 is to prevent underflow
        score += (1 + self_throw + (self_throw + 4 - opponent_throw) % 3 * 3) as u32;
    }
    println!("{}", score);
}
