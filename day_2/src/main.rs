use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut score_part_1 = 0;
    let mut score_part_2 = 0;
    for line in lines {
        let line = line.unwrap();
        let line = line.as_bytes();

        // We can represent the throws as rock = 0, paper = 1, and scissors = 2
        let opponent_throw = line[0] - 'A' as u8;
        // For part 1 self_throw has the same interpretation as opponent_throw
        // For part 2 a win is 2, a draw is 1, and a loss is 0
        let self_throw = line[2] - 'X' as u8;
        // Taking a difference %3 can give us the result of a game
        // Adding 4 instead of 1 is to prevent underflow
        score_part_1 += (1 + self_throw + (self_throw + 4 - opponent_throw) % 3 * 3) as u32;
        // This time we use the sum %3 to calculate what was thrown
        score_part_2 += (self_throw * 3 + 1 + (opponent_throw + self_throw + 2) % 3) as u32;
    }
    println!("{}", score_part_1);
    println!("{}", score_part_2);
}
