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
        // This time a win is 2, a draw is 1, and a loss is 0
        let self_throw =  line[2] - 'X' as u8;
        // This time we use the sum %3 to calculate what was thrown
        score += (self_throw * 3 + 1 + (opponent_throw + self_throw + 2) % 3) as u32;
    }
    println!("{}", score);
}
