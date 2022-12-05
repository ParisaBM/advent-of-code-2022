use std::fs::File;
use std::io::{self, BufRead};
use std::mem::swap;
use std::path::Path;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut highest_sums = [0; 3];
    let mut current_sum = 0;
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            for i in 0..3 {
                if current_sum > highest_sums[i] {
                    swap(&mut current_sum, &mut highest_sums[i]);
                }
            }
            current_sum = 0;
        } else {
            current_sum += line.parse::<i32>().unwrap();
        }
    }
    for i in 0..3 {
        if current_sum > highest_sums[i] {
            swap(&mut current_sum, &mut highest_sums[i]);
        }
    }
    println!("{}", highest_sums.iter().sum::<i32>());
}
