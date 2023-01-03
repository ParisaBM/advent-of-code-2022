use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut valve_to_int = HashMap::new();
    let mut valve_pressures = HashMap::new();
    let mut paths = HashMap::new();
    for line in lines {
        let tokens = line.unwrap().split_whitespace().map(|x| x.to_string()).collect::<Vec<_>>();
        let name = tokens[1].clone();
        valve_to_int.insert(name.clone(), valve_to_int.len());
        let rate_token = tokens[4].clone();
        let rate = rate_token[5..rate_token.len()-1].parse::<u32>().unwrap();
        valve_pressures.insert(name.clone(), rate);
        paths.insert(name.clone(), Vec::new());
        for destination in tokens.iter().skip(9) {
            let destination = if destination.chars().last().unwrap() == ',' {
                destination[..destination.len()-1].to_string()
            } else {
                destination.to_string()
            };
            paths.get_mut(&name).unwrap().push(destination);
        }
    }
    println!("{:?}\n{:?}\n{:?}", valve_to_int, valve_pressures, paths);
}
