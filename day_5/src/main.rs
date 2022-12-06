use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let mut stacks_part_1 = Vec::new();
    // This first loop reads up to the blank line
    // It sets up the starting stacks
    for line in &mut lines {
        let line = line.unwrap();
        // Once we see a blank line we're on to the second section
        if line.is_empty() {
            break;
        }
        for (i, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                while stacks_part_1.len() < i / 4 + 1 {
                    stacks_part_1.push(Vec::new());
                }
            }
            if c.is_alphabetic() {
                stacks_part_1[i / 4].push(c);
            }
        }
    }
    // Since the stacks are lifo, they'll end up in reverse
    for stack in stacks_part_1.iter_mut() {
        stack.reverse();
    }
    // Parts 1 and 2 use the same starting stack, just the operations are different
    let mut stacks_part_2 = stacks_part_1.clone();

    // Here we handle all the moves
    for line in lines {
        let line = line.unwrap();
        // For each move, the important parameters are the numeric values which appear as every other word
        // This next statement takes those tokens, converts them to ints and collects them
        let parameters = line
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        // Parts 1 and 2 use the same parameters
        for _ in 0..parameters[0] {
            let top = stacks_part_1[parameters[1] - 1].pop().unwrap();
            stacks_part_1[parameters[2] - 1].push(top);
        }
        let final_len = stacks_part_2[parameters[1] - 1].len() - parameters[0];
        let mut top = stacks_part_2[parameters[1] - 1].split_off(final_len);
        stacks_part_2[parameters[2] - 1].append(&mut top);
    }
    println!(
        "Part 1: {}",
        stacks_part_1
            .iter()
            .filter_map(|x| x.last())
            .collect::<String>()
    );
    println!(
        "Part 2: {}",
        stacks_part_2
            .iter()
            .filter_map(|x| x.last())
            .collect::<String>()
    );
}
