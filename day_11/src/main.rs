use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The 2 operations that monkeys apply to the worry level
#[derive(Clone)]
enum Operator {
    Add,
    Multiply,
}

// The operations apply to some combination of 'old' and integer constants
// The first operand is always 'old'
#[derive(Clone)]
enum Operand {
    Old,
    Const(u64),
}

// Contains all the properties of a monkey and the items they're carrying
#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operator: Operator,
    operand: Operand,
    divisibility_test: u64,
    true_target: usize,
    false_target: usize,
    items_inspected: u128,
}

fn gcf(x: u64, y: u64) -> u64 {
    if y == 0 {
        x
    } else {
        gcf(y, x % y)
    }
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcf(x, y)
}

fn main() {
    let file = File::open(Path::new("input")).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                .map(|x| x.to_owned())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut monkeys = Vec::new();
    // Each monkey takes 6 lines to describe, and then there's a blank between each
    // This loop ingests lines 7 at a time
    for group in lines.chunks(7) {
        // Now we can begin assembling a monkey using the values from the group
        let items = group[1]
            .iter()
            .skip(2)
            .map(|x| {
                (if x.ends_with(",") {
                    x[..x.len() - 1].to_string()
                } else {
                    x.to_string()
                })
                .parse::<u64>()
                .unwrap()
            })
            .collect();
        let operator = match group[2][4].as_str() {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!(),
        };
        let operand = match group[2][5].as_str() {
            "old" => Operand::Old,
            x => Operand::Const(x.parse::<u64>().unwrap()),
        };
        let divisibility_test = group[3][3].to_owned().parse::<u64>().unwrap();
        let true_target = group[4][5].to_owned().parse::<usize>().unwrap();
        let false_target = group[5][5].to_owned().parse::<usize>().unwrap();
        monkeys.push(Monkey {
            items,
            operator,
            operand,
            divisibility_test,
            true_target,
            false_target,
            items_inspected: 0,
        });
    }
    // All item values can be taken modulo the lcm of all the divisibility tests
    // For example if there are 2 monkeys with tests of 11 and 13, we can take worry values module 11*13=143
    let modulus = monkeys
        .iter()
        .fold(1, |prev, monkey| lcm(prev, monkey.divisibility_test));
    for part in 1..=2 {
        let backup_monkeys = monkeys.clone();
        let iterations = [20, 10000][part - 1];
        for _ in 0..iterations {
            for i in 0..monkeys.len() {
                for j in 0..monkeys[i].items.len() {
                    let item = monkeys[i].items[j];
                    let rvalue = match monkeys[i].operand {
                        Operand::Const(x) => x,
                        Operand::Old => item,
                    };
                    let new_value = match monkeys[i].operator {
                        Operator::Add => item + rvalue,
                        Operator::Multiply => item * rvalue,
                    } % modulus
                        / if part == 1 { 3 } else { 1 };
                    let target = if new_value % monkeys[i].divisibility_test == 0 {
                        monkeys[i].true_target
                    } else {
                        monkeys[i].false_target
                    };
                    monkeys[target].items.push(new_value);
                    monkeys[i].items_inspected += 1;
                }
                // Now that all the items have been thrown, we don't want duplicates of them
                monkeys[i].items.clear();
            }
        }
        let mut inspections = monkeys
            .iter()
            .map(|x| x.items_inspected)
            .collect::<Vec<_>>();
        inspections.sort();
        println!(
            "Part {}: {}",
            part,
            inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
        );
        monkeys = backup_monkeys;
    }
}
