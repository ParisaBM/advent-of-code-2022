use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut priority_sum = 0;
    for line in lines {
        let line = line.unwrap();
        let line = line.as_bytes();

        // This array tracks which letters we find in the first half
        priority_sum += find_overlap(&[&line[..line.len() / 2], &line[line.len() / 2..]])
    }
    println!("Part 1: {}", priority_sum);

    let file = File::open(Path::new("input")).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let mut priority_sum = 0;
    while let Some(Ok(s0)) = lines.next() {
        let s1 = lines.next().unwrap().unwrap();
        let s2 = lines.next().unwrap().unwrap();
        priority_sum += find_overlap(&[s0.as_bytes(), s1.as_bytes(), s2.as_bytes()]);
    }
    println!("Part 2: {}", priority_sum);
}

// given strings, it finds the letter shared by all of them, and returns its priority value
fn find_overlap(strings: &[&[u8]]) -> usize {
    // instances counts the number of strings a letter has been found in
    let mut instances = [0; 52];
    for s in strings {
        // already_counted prevents us from double-counting duplicate letters in a string
        let mut already_counted = [false; 52];
        for c in *s {
            let lp = letter_priority(*c);
            if !already_counted[lp - 1] {
                instances[lp - 1] += 1;
                if instances[lp - 1] == strings.len() {
                    return lp;
                }
                already_counted[lp - 1] = true;
            }
        }
    }
    panic!();
}

fn letter_priority(c: u8) -> usize {
    (if c <= 'Z' as u8 {
        26 + c - 'A' as u8
    } else {
        c - 'a' as u8
    }) as usize
        + 1
}
