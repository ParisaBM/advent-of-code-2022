use std::fs;

// SHAPES describes the shape of each rock given as a list of points
// The tuples are given (y, x) where the y axis points up, and x points right
// 0 is the lower and left edges respectively
const SHAPES: &[&[(usize, usize)]] = &[
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
    &[(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(0, 0), (0, 1), (1, 0), (1, 1)],
];

// This algorithm makes special use of the data I was given
// Recall the left/right directions loop
// In this data set after the first loop the chamber grows by the same height and adds the same number of pieces per loop
// After two loops we know how much each of these increase by, thus allowing us to skip many loops
// During the third loop, when we get to the same point in the cycle as a trillion, we can add the current height to amount that would be added in the skipped cycles
// This amount we add is the height increase per loop, multiplied by the number of loops that were skipped
// For 2022 we can just simulate this many rocks one at a time

fn main() {
    let file = fs::read_to_string("input").unwrap();
    // directions encode the rock movements as 1 for right and -1 for left
    let directions = file.chars().map(|x| match x {
        '<' => -1,
        '>' => 1,
        _ => panic!(),
    });
    let loop_len = file.len();

    // We represent the chamber as a 2d array of booleans, it has width 7
    let mut chamber = vec![[false; 7]; 4];
    // chamber_height is the number of layers of rock that have acrued
    let mut chamber_height = 0;

    // rock can either be thought as the number of solid rocks, or the current rock number
    let mut rock = 0;
    // these coordiantes give the lower left corner of the rock
    let (mut y, mut x) = (3, 2);
    let mut shape = SHAPES[0];
    // Shape height will tell us how much the overall chamber height should increase once the rock drops
    let mut shape_height = 1;

    // These are our afformentioned metrics of chamber growth after 1 and 2 passes through the moves
    let mut one_loop_pieces = 0;
    let mut one_loop_height = 0;
    let mut two_loop_pieces = 0;
    let mut two_loop_height = 0;
    for (i, direction) in directions.cycle().enumerate() {
        if i == loop_len {
            one_loop_height = chamber_height;
            one_loop_pieces = rock;
        }
        if i == loop_len * 2 {
            two_loop_height = chamber_height;
            two_loop_pieces = rock;
        }
        // This loop determines if the rock can move in the given direction
        let mut valid_direction = true;
        for piece_offset in shape.iter() { // We check the rock segments one at a time
            let piece = (piece_offset.0 + y, (piece_offset.1 + x) as isize + direction); 
            if piece.1 < 0 || piece.1 >= 7 || chamber[piece.0][piece.1 as usize] {
                valid_direction = false;
                break;
            }
        }
        // Move the rock if applicable
        if valid_direction {
            x = (x as isize + direction) as usize;
        }
        // We similarly check if the rock can fall
        let mut valid_fall = true;
        for piece_offset in shape.iter() {
            let piece = ((piece_offset.0 + y) as isize - 1, piece_offset.1 + x); 
            if piece.0 < 0 || chamber[piece.0 as usize][piece.1] {
                valid_fall = false;
                break;
            }
        }
        if valid_fall {
            y -= 1;
        } else {
            // If the piece cannot fall we begin the descent of the next rock
            chamber_height = chamber_height.max(y + shape_height);
            // This loop adds the rock in its final position to the grid
            for piece_offset in shape.iter() {
                let piece = (piece_offset.0 + y, piece_offset.1 + x);
                chamber[piece.0][piece.1] = true;
            }
            // Now we move to the next rock and get its shape and height
            rock += 1;
            shape = SHAPES[rock % 5];
            shape_height = shape.iter().map(|x| x.0).max().unwrap() + 1;
            // We make sure the chamber array is tall enough to fit the rock in its starting position
            while chamber.len() < chamber_height + 3 + shape_height {
                chamber.push([false; 7]);
            }
            if rock == 2022 {
                println!("Part 1: {}", chamber_height);
            } else if i > loop_len * 2 && (1_000_000_000_000 - rock) % (two_loop_pieces - one_loop_pieces) == 0 {
                // The second condition can be thought of as "if the number of remaining rock a multiple of the amount added per loop"
                // We add to the current height the number of such cycles times the growth per cycle
                println!("Part 2: {}", chamber_height + (1_000_000_000_000 - rock) / (two_loop_pieces - one_loop_pieces) * (two_loop_height - one_loop_height));
                break;
            }
            // This is the given starting position for each rock
            (y, x) = (chamber_height + 3, 2);
        }
    }
}