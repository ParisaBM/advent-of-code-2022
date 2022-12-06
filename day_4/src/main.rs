use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut total_containments = 0;
    let mut total_overlaps = 0;
    for line in lines {
        let line = line.unwrap();
        let delimiters = line
            .split(|c| c == ',' || c == '-')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        // We use the fact that a product is non-negative iff the multiplicands have matching sign
        if (delimiters[2] - delimiters[0]) * (delimiters[1] - delimiters[3]) >= 0 {
            total_containments += 1;
        }
        if (delimiters[1] >= delimiters[2]) && (delimiters[3] >= delimiters[0]) {
            total_overlaps += 1;
        }
    }
    println!("Part 1: {}", total_containments);
    println!("Part 2: {}", total_overlaps);
}
