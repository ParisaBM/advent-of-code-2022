use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut highest_sum = 0;
    let mut current_sum = 0;
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            highest_sum = highest_sum.max(current_sum);
            current_sum = 0;
        } else {
            current_sum += line.parse::<i32>().unwrap();
        }
    }
    println!("{}", highest_sum);
}
