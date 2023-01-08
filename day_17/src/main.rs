use std::fs;

const SHAPES: &[&[(usize, usize)]] = &[
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
    &[(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(0, 0), (0, 1), (1, 0), (1, 1)],
];

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let mut directions = file.chars().cycle();
    let mut chamber = Vec::new();
    let mut chamber_height = 0;
    for rock in 0..1_000_000 {
        let shape = SHAPES[rock % SHAPES.len()];
        let shape_height = shape.iter().map(|x| x.0).max().unwrap() + 1;
        while chamber.len() < chamber_height + 3 + shape_height {
            chamber.push([false; 7]);
        }
        let (mut y, mut x) = (chamber_height + 3, 2);
        loop {
            let direction = match directions.next().unwrap() {
                '<' => -1,
                '>' => 1,
                _ => panic!(),
            };
            let mut valid_direction = true;
            for piece_offset in shape.iter() {
                let piece = (piece_offset.0 + y, (piece_offset.1 + x) as isize + direction); 
                if piece.1 < 0 || piece.1 >= 7 || chamber[piece.0][piece.1 as usize] {
                    valid_direction = false;
                    break;
                }
            }
            if valid_direction {
                x = (x as isize + direction) as usize;
            }
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
                break;
            }
        }
        chamber_height = chamber_height.max(y + shape_height);
        for piece_offset in shape.iter() {
            let piece = (piece_offset.0 + y, piece_offset.1 + x);
            chamber[piece.0][piece.1] = true;
        } 
    }
    println!("{}", chamber_height);
}