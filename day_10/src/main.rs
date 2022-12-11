use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut register_x = 1;
    let mut cycle = 1_i32;
    let mut signal_strength = 0;
    let mut crt = Vec::new();
    for line in lines {
        let line = line.unwrap();

        let mut tokens = line.split_whitespace();
        let instruction = tokens.next().unwrap();
        // instruction_cycles is how many cycles the instruction takes to execute
        let instruction_cycles = match instruction {
            "addx" => 2,
            "noop" => 1,
            _ => panic!("bad instruction"),
        };
        // Since an addx takes 2 cycles, if it ends on 19
        if cycle % 40 == 20 || instruction == "addx" && cycle % 40 == 19 {
            // We always multiple by a multiple of 20, even if the cycle number isn't
            let measurement_cycle = cycle + cycle % 2;
            signal_strength += measurement_cycle * register_x;
        }
        // This loop updates the CRT once or twice
        for i in 0..instruction_cycles {
            // We add a new row every 40 cycles
            // Cycles are 1-indexed
            if (cycle + i) % 40 == 1 {
                crt.push(String::new());
            }
            crt.last_mut()
                .unwrap()
                .push_str(if ((cycle + i - 1) % 40 - register_x).abs() <= 1 {
                    "#"
                } else {
                    "."
                });
        }
        // The register changes happen at the end of the second cycle
        if instruction == "addx" {
            register_x += tokens.next().unwrap().parse::<i32>().unwrap();
        }
        cycle += instruction_cycles;
    }
    println!("Part 1: {}", signal_strength);
    println!("Part 2:");
    for row in crt.iter() {
        println!("{}", row);
    }
}
