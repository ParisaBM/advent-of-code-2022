use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut heights = Vec::new();
    for line in lines {
        let line = line.unwrap();

        heights.push(line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>());
    }
    // counted makes sure we don't count trees from multiple directions
    let mut counted = vec![vec![false; heights[0].len()]; heights.len()];
    let mut visible_trees = 0;
    // left to right
    for i in 0..heights.len() {
        let mut heighest = -1;
        for j in 0..heights[i].len() {
            if heights[i][j] > heighest {
                counted[i][j] = true;
                heighest = heights[i][j];
                visible_trees += 1;
            }
        }
    }
    // right to left
    for i in 0..heights.len() {
        let mut heighest = -1;
        for j in (0..heights[i].len()).rev() {
            if heights[i][j] > heighest {
                heighest = heights[i][j];
                if !counted[i][j] {
                    counted[i][j] = true;
                    visible_trees += 1;
                }
            }
        }
    }
    // top to bottom
    for j in 0..heights[0].len() {
        let mut heighest = -1;
        for i in 0..heights.len() {        
            if heights[i][j] > heighest {
                heighest = heights[i][j];
                if !counted[i][j] {
                    counted[i][j] = true;
                    visible_trees += 1;
                }
            }
        }
    }
    // bottom to top
    for j in 0..heights[0].len() {
        let mut heighest = -1;
        for i in (0..heights.len()).rev() {        
            if heights[i][j] > heighest {
                heighest = heights[i][j];
                if !counted[i][j] {
                    counted[i][j] = true;
                    visible_trees += 1;
                }
            }
        }
    }
    println!("Part 1: {:?}", visible_trees);
    let mut most_scenic = 0;
    for i in 1..heights.len()-1 {
        for j in 1..heights[i].len()-1 {
            let mut scenic_score = 1; // 1 because it's a product
            // right
            for k in 1.. {
                if k == heights[i].len()-j-1 || heights[i][j+k] >= heights[i][j] {
                    scenic_score *= k;
                    break;
                }
            }
            // left
            for k in 1.. {
                if k == j || heights[i][j-k] >= heights[i][j] {
                    scenic_score *= k;
                    break;
                }
            }
            // down
            for k in 1.. {
                if k == heights.len()-i-1 || heights[i+k][j] >= heights[i][j] {
                    scenic_score *= k;
                    break;
                }
            }
            // up
            for k in 1.. {
                if k == i || heights[i-k][j] >= heights[i][j] {
                    scenic_score *= k;
                    break;
                }
            }
            most_scenic = most_scenic.max(scenic_score);
        }
    }
    println!("Part 2: {}", most_scenic);
}
