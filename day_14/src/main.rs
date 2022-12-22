use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    // paths is a vector of vectors of tuples
    // The first index gives the path, then the vertex in the path
    let paths = io::BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                // every other token is an arrow, we skip these
                .step_by(2)
                .map(|x| {
                    x.split_terminator(',')
                        .map(|x| x.parse::<isize>().unwrap())
                        // collects the position
                        .collect_tuple::<(isize, isize)>()
                        .unwrap()
                })
                // collects the path
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Now we find the bottom-, left-, and rightmost points
    // The left bound will be somewhere around 500, and let us start there instead of having wasted cells
    // The right bound gives us the width
    // The bottom bound gives us the number of rows
    // We pad bottom by two for the last row
    // We pad the sides based on the height for overflow sand
    let bottom = paths
        .iter()
        .flat_map(|x| x.iter())
        .map(|x| x.1)
        .max()
        .unwrap() as usize
        + 2;
    let left = paths
        .iter()
        .flat_map(|x| x.iter())
        .map(|x| x.0)
        .min()
        .unwrap() as usize
        - bottom;
    let right = paths
        .iter()
        .flat_map(|x| x.iter())
        .map(|x| x.0)
        .max()
        .unwrap() as usize
        + bottom;

    // Now we know the dimensions for the grid
    // We use false to represent an empty space, and true to represent a filled space
    let mut grid = vec![vec![false; bottom + 1]; right - left + 1];

    // Now we can add the starting walls
    for path in paths.iter() {
        for i in 0..path.len() - 1 {
            let (start, end) = (path[i], path[i + 1]);
            let direction = ((end.0 - start.0).signum(), (end.1 - start.1).signum());
            for j in 0.. {
                // fill is where we need to add a rock
                let fill = (start.0 + direction.0 * j, start.1 + direction.1 * j);
                grid[fill.0 as usize - left][fill.1 as usize] = true;
                if fill == end {
                    break;
                }
            }
        }
    }
    // And the floor
    for row in grid.iter_mut() {
        *row.last_mut().unwrap() = true;
    }

    // Now we add the sand, once sand has settled we can treat it as a rock
    // Part 1 finishes once the first piece of sand reaches the bottom
    // Part 2 will keep going from here
    let mut part_1_finished = false;
    // The sand_stack allows each piece of sand to start at the last empty space on the previous sand's spot
    let mut sand_stack = vec![(500 - left, 0)];
    'outer: for i in 0.. {
        // i counts the grains of sand
        let mut sand = sand_stack.pop().unwrap();
        loop {
            // This advances the sand one step if possible
            if !grid[sand.0][sand.1 + 1] {
                sand_stack.push(sand);
                sand.1 += 1;
            } else if !grid[sand.0 - 1][sand.1 + 1] {
                sand_stack.push(sand);
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !grid[sand.0 + 1][sand.1 + 1] {
                sand_stack.push(sand);
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                // This means it has gotten stuck
                // Part 1 finishes if the sand gets stuck at the bottom
                // Part 2 finishes if the sand gets stuck at the top
                if sand.1 == bottom - 1 && !part_1_finished {
                    println!("Part 1: {}", i);
                    part_1_finished = true;
                }
                if sand.1 == 0 {
                    println!("Part 2: {}", i + 1);
                    break 'outer;
                }
                grid[sand.0][sand.1] = true;
                break;
            }
        }
    }
}
