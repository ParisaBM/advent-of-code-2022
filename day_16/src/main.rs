use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::{HashMap, VecDeque};
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file).lines();
    // The vavlves are given by a pair of letters
    // Later stages of the algorithm will assume they are given by integers starting from 0
    // They're numbered based on which line in the file describes them i.e. the first line gives valve 0, etc.
    // valve_to_int is a map that allows us to make this conversion
    let mut valve_to_int = HashMap::new();
    // valve_rates gives the rate of release from its index, not its alphabetic name
    let mut valve_rates = Vec::new();
    // paths maps each valve given by its index to its connected valves, which are give by name
    let mut paths = Vec::new();
    for line in lines {
        let tokens = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        // name is the name of the valve describe on the current line
        let name = tokens[1].clone();
        // by mapping name to valve_to_int.len(), we assign integers sequentially
        valve_to_int.insert(name.clone(), valve_to_int.len());
        let rate_token = tokens[4].clone();
        let rate = rate_token[5..rate_token.len() - 1].parse::<i32>().unwrap();
        valve_rates.push(rate);
        paths.push(Vec::new());
        for destination in tokens.iter().skip(9) {
            let destination = if destination.chars().last().unwrap() == ',' {
                destination[..destination.len() - 1].to_string()
            } else {
                destination.to_string()
            };
            paths.last_mut().unwrap().push(destination);
        }
    }
    // We now use valve_to_int to translate paths so the destinations are also given by their index
    let paths = paths
        .iter()
        .map(|x| x.iter().map(|x| valve_to_int[x]).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // We always start from the valve labeled AA
    let start = valve_to_int[&String::from("AA")];
    // At this point, the valve names can be fully ignored

    // The distance_matrix is the number of steps to get from any valve to any other valve
    // -1 is a placeholder value
    let mut distance_matrix = vec![vec![-1; valve_to_int.len()]; valve_to_int.len()];
    // To compute the above matrix, we do the following:
    // We have each pair (i, j) such that the distance from i to j is known put into a queue
    // As we pull (i, j) pairs from the end of the queue we consider each k s.t. k is reachable from j
    // If a path from i to k hasn't been found, then its shortest path is one longer than the distance between i and j
    // By using a queue instead of say, a stack, the first path we find will always be the shortest
    // We can initialize the queue by setting the distance from any valve to itself 0
    let mut connection_queue = VecDeque::new();
    for i in 0..valve_to_int.len() {
        distance_matrix[i][i] = 0;
        connection_queue.push_back((i, i));
    }
    while let Some((i, j)) = connection_queue.pop_front() {
        for k in paths[j].iter() {
            if distance_matrix[i][*k] == -1 {
                distance_matrix[i][*k] = distance_matrix[i][j] + 1;
                connection_queue.push_back((i, *k));
            }
        }
    }

    // We know that any optimal solution will consist of a cycle of:
    // 1. Traveling to a valve with positive pressure via a shortest path
    // 2. Opening it
    // Thus a solution can be given only by the order in which its valves are opened
    // Although the space of possible such orderings is very large (exponentially so),
    // it's much smaller than the space of possible sequences of actions as described in the rules

    // Now that the distance between any two valves is known, we can prune any valves with a pressure of 0
    // The only exception to this is the starting node which we'll want to keep
    // reduced_matrix has all the these distance except with these rows and columns removed
    let mut reduced_matrix = Vec::new();
    for i in 0..valve_to_int.len() {
        if i != start && valve_rates[i] == 0 {
            continue;
        }
        reduced_matrix.push(Vec::new());
        for j in 0..valve_to_int.len() {
            if j != start && valve_rates[j] == 0 {
                continue;
            }
            reduced_matrix
                .last_mut()
                .unwrap()
                .push(distance_matrix[i][j]);
        }
    }
    // We also remove the rate values for the pruned valves
    valve_rates = valve_rates
        .iter()
        .enumerate()
        .filter(|(i, x)| *i == start || **x != 0)
        .map(|(_, x)| *x)
        .collect();
    // With this pruning, the starting valve may have a different index
    let start = valve_rates.iter().position(|x| *x == 0).unwrap();

    // Now we just call the functions that give the solutions for parts 1 and 2 based on the data structures we have
    // We begin start, so its initialized to visited, details later
    let mut visited = vec![false; valve_rates.len()];
    visited[start] = true;
    println!(
        "Part 1: {}",
        max_release(
            &reduced_matrix,
            &valve_rates,
            30,
            &mut visited,
            start
        )
    );
    println!(
        "Part 2: {}",
        max_release_2_agents(
            &reduced_matrix,
            &valve_rates,
            26,
            &mut visited,
            start,
            start
        )
    );
}

// max_release gives the most amount of pressure that can be released by one agent given a set of constraints
// matrix is the distance between any 2 valves
// rates is the rate of pressure release for each valve
// time is the remaining time
// visited gives which which valves have already been opened so they not be opened twice
// The visted list is never duplicated, instead a shared mutable reference to list is used
// As the function recursively descends more locations are marked as visited, which is reveresed as the stack unwinds
// As such, the reference is effectively immutable as any changes to it are undone
// location is the current location of the agent
// The function itself recursively considers all possible orderings of the valves to find the optimal output
fn max_release(
    matrix: &Vec<Vec<i32>>,
    rates: &Vec<i32>,
    time: i32,
    visited: &mut Vec<bool>,
    location: usize,
) -> i32 {
    let mut optimum = 0;
    for next in 0..visited.len() {
        if visited[next] {
            continue;
        }
        // The time taken is the travel time, plus one minute to open to open the valve
        let new_time = time - matrix[location][next] - 1;
        if new_time <= 0 {
            continue;
        }
        // When a valve is openend we can immediately add the full amount of pressure it will release
        visited[next] = true;
        let score =
            rates[next] * new_time + max_release(matrix, rates, new_time, visited, next);
        visited[next] = false;
        optimum = optimum.max(score);
    }
    return optimum;
}

// This function is for part 2 where we have 2 agents
// It's largely the same as the previous function, except that once the path for one agent is contstructed,
// we can switch over to the other agent to construct their path
// For this, we call the max_release function
// In this call, we reset the time back to 26 as both start at the same time
// We DON'T reset visited, as each location can only be visited once by either
fn max_release_2_agents(
    matrix: &Vec<Vec<i32>>,
    rates: &Vec<i32>,
    time: i32,
    visited: &mut Vec<bool>,
    location: usize,
    starting_location: usize,
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
        let score = rates[next] * new_time
            + max_release_2_agents(matrix, rates, new_time, visited, next, starting_location);
            optimum = optimum.max(score);
        visited[next] = false;
    }
    let score = max_release(matrix, rates, 26, visited, starting_location);
    optimum = optimum.max(score);
    return optimum;
}
