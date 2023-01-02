use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashSet;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    // let lines = io::BufReader::new(file).lines().map(|x| x.unwrap().split(|x: char| !x.is_numeric()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let digitless = Regex::new(r"[^\d]+").unwrap();
    let mut beacons_on_row = HashSet::new();
    let mut intervals = io::BufReader::new(file)
        .lines()
        .map(|x| {
            digitless.split(&x.unwrap()).skip(1)
                .map(|x| x.to_owned().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|x| {
            println!("{:?}", x);
            let beacon_distance = (x[0] - x[2]).abs() + (x[1] - x[3]).abs();
            let row_distance = (x[1] - 2000000).abs();
            if x[3] == 2000000 {
                beacons_on_row.insert(x[2]);
            }
            return (x[0] - beacon_distance + row_distance, x[0] + beacon_distance - row_distance);
        }).filter(|x| x.0 <= x.1)
        .collect::<Vec<_>>();
    for interval in intervals.iter() {
        println!("{:?}", interval);
    }
    intervals.sort_by_key(|x| x.0);
    println!();
    for interval in intervals.iter() {
        println!("{:?}", interval);
    }
    let mut coverage = 0;
    let mut furthest = -10_000_000;
    for (left, right) in intervals.iter() {
        coverage += (right - left.max(&(furthest + 1)) + 1).max(0);
        furthest = furthest.max(*right);
        println!("{}", coverage);
    }
    println!("{:?}", beacons_on_row);
    println!("{}", coverage - beacons_on_row.len() as i32);
}
