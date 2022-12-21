use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut elavation = Vec::new();
    let mut start_position = (0, 0);
    let mut end_position = (0, 0);
    for (y, line) in lines.into_iter().enumerate() {
        let line = line.unwrap();

        elavation.push(Vec::new());
        for (x, c) in line.chars().enumerate() {
            elavation.last_mut().unwrap().push(match c {
                'S' => {
                    start_position = (x, y);
                    0
                }
                'E' => {
                    end_position = (x, y);
                    25
                }
                c => c as u8 - 'a' as u8,
            })
        }
    }
    let mut cost = vec![vec![None; elavation[0].len()]; elavation.len()];
    cost[end_position.1][end_position.0] = Some(0);
    let mut location_queue = VecDeque::from([end_position]);
    let mut min_a_cost = 1000;
    while let Some(next) = location_queue.pop_front() {
        let next_cost = cost[next.1][next.0].unwrap();
        if elavation[next.1][next.0] == 0 {
            min_a_cost = min_a_cost.min(next_cost)
        }
        if next == start_position {
            println!("Part 1: {}", next_cost);
            break;
        }
        if next.0 != 0
            && cost[next.1][next.0 - 1] == None
            && elavation[next.1][next.0] <= elavation[next.1][next.0 - 1] + 1
        {
            cost[next.1][next.0 - 1] = Some(next_cost + 1);
            location_queue.push_back((next.0 - 1, next.1));
        }
        if next.0 != elavation[0].len() - 1
            && cost[next.1][next.0 + 1] == None
            && elavation[next.1][next.0] <= elavation[next.1][next.0 + 1] + 1
        {
            cost[next.1][next.0 + 1] = Some(next_cost + 1);
            location_queue.push_back((next.0 + 1, next.1));
        }
        if next.1 != 0
            && cost[next.1 - 1][next.0] == None
            && elavation[next.1][next.0] <= elavation[next.1 - 1][next.0] + 1
        {
            cost[next.1 - 1][next.0] = Some(next_cost + 1);
            location_queue.push_back((next.0, next.1 - 1));
        }
        if next.1 != elavation.len() - 1
            && cost[next.1 + 1][next.0] == None
            && elavation[next.1][next.0] <= elavation[next.1 + 1][next.0] + 1
        {
            cost[next.1 + 1][next.0] = Some(next_cost + 1);
            location_queue.push_back((next.0, next.1 + 1));
        }
    }
    println!("Part 2: {}", min_a_cost);
}
