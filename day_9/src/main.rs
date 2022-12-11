use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut rope_segments = [(0_i32, 0_i32); 10];
    // The movement is lead with rope_segment[0]
    // The movement of rope_segment[1] gives us the sum for part 1
    // The movement of rope_segment[9] gives us the sum for part 2
    let mut visited_part_1 = HashSet::new();
    let mut visited_part_2 = HashSet::new();
    for line in lines {
        let line = line.unwrap();

        let mut tokens = line.split_whitespace();
        let direction = tokens.next().unwrap();
        let distance = tokens.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..distance {
            // This part moves the head
            match direction {
                "R" => rope_segments[0].0 += 1,
                "L" => rope_segments[0].0 -= 1,
                "U" => rope_segments[0].1 += 1, 
                "D" => rope_segments[0].1 -= 1,
                _ => panic!("bad direction"),
            }
            // This loop causes all the tails to follow
            for i in 0..9 {
                if (rope_segments[i].0 - rope_segments[i+1].0).abs() >= 2 || (rope_segments[i].1 - rope_segments[i+1].1).abs() >= 2 {
                    rope_segments[i+1].0 += (rope_segments[i].0 - rope_segments[i+1].0).signum();
                    rope_segments[i+1].1 += (rope_segments[i].1 - rope_segments[i+1].1).signum();
                }    
            }
            // We add at the end of each iteration
            // This doesn't omit the starting position because the tail doesn't move on the first iteration
            visited_part_1.insert(rope_segments[1]);
            visited_part_2.insert(rope_segments[9]);
        }
    }
    println!("Part 1: {}", visited_part_1.len());
    println!("Part 2: {}", visited_part_2.len());
}
