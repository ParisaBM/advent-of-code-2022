use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::{HashMap, VecDeque};
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut valve_to_int = HashMap::new();
    let mut valve_pressures = Vec::new();
    let mut paths = HashMap::new();
    for line in lines {
        let tokens = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let name = tokens[1].clone();
        valve_to_int.insert(name.clone(), valve_to_int.len());
        let rate_token = tokens[4].clone();
        let rate = rate_token[5..rate_token.len() - 1].parse::<i32>().unwrap();
        valve_pressures.push(rate);
        paths.insert(name.clone(), Vec::new());
        for destination in tokens.iter().skip(9) {
            let destination = if destination.chars().last().unwrap() == ',' {
                destination[..destination.len() - 1].to_string()
            } else {
                destination.to_string()
            };
            paths.get_mut(&name).unwrap().push(destination);
        }
    }
    println!("{:?}\n{:?}\n{:?}", valve_to_int, valve_pressures, paths);
    let mut distance_matrix = vec![vec![-1; valve_to_int.len()]; valve_to_int.len()];
    let mut connection_queue = VecDeque::new();
    for i in 0..valve_to_int.len() {
        distance_matrix[i][i] = 0;
    }
    for valve in valve_to_int.keys() {
        connection_queue.push_back((valve, valve));
    }
    while let Some((v0, v1)) = connection_queue.pop_front() {
        let i = valve_to_int[v0];
        let j = valve_to_int[v1];
        for destination in paths[v1].iter() {
            let k = valve_to_int[destination];
            if distance_matrix[i][k] == -1 {
                distance_matrix[i][k] = distance_matrix[i][j] + 1;
                connection_queue.push_back((v0, destination));
            }
        }
    }
    let start = valve_to_int[&String::from("AA")];
    let mut reduced_matrix = Vec::new();
    for i in 0..valve_to_int.len() {
        if i != start && valve_pressures[i] == 0 {
            continue;
        }
        reduced_matrix.push(Vec::new());
        for j in 0..valve_to_int.len() {
            if j != start && valve_pressures[j] == 0 {
                continue;
            }
            reduced_matrix
                .last_mut()
                .unwrap()
                .push(distance_matrix[i][j]);
        }
    }
    for row in reduced_matrix.iter() {
        println!("{:?}", row);
    }
    valve_pressures = valve_pressures
        .iter()
        .enumerate()
        .filter(|(i, x)| *i == start || **x != 0)
        .map(|(_, x)| *x)
        .collect();
    let start = valve_pressures.iter().position(|x| *x == 0).unwrap();
    println!("{:?}", valve_pressures);
    println!("{}", max_release(&reduced_matrix, &valve_pressures, 30, &mut vec![false; valve_pressures.len()], start));
}

fn max_release(
    matrix: &Vec<Vec<i32>>,
    pressure: &Vec<i32>,
    time: i32,
    visited: &mut Vec<bool>,
    location: usize,
) -> i32 {
    let mut optimum = 0;
    for next in 0..visited.len() {
        if visited[next] {
            continue;
        }
        let new_time = time - matrix[location][next] - 1;
        if new_time <= 0 {
            continue;
        }
        visited[next] = true;
        let score = pressure[next] * new_time + max_release(matrix, pressure, new_time, visited, next);
        visited[next] = false;
        optimum = optimum.max(score);
    }
    return optimum;
}
