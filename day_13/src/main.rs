use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// We parse packets by first turning them into a sequence of tokens, and then parsing that
enum Token {
    OpenBracket,
    CloseBracket,
    Integer(u32),
}

impl Token {
    // This turns a string into a sequence of tokens
    fn from_str(s: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        // buffer accumulates digits in integers
        let mut buffer = String::new();
        for c in s.chars() {
            if c.is_digit(10) {
                buffer.push(c);
            } else {
                if !buffer.is_empty() {
                    tokens.push(Token::Integer(buffer.parse::<u32>().unwrap()));
                    buffer.clear();
                }
                if c == '[' {
                    tokens.push(Token::OpenBracket);
                }
                if c == ']' {
                    tokens.push(Token::CloseBracket);
                }
            }
        }
        if !buffer.is_empty() {
            tokens.push(Token::Integer(buffer.parse::<u32>().unwrap()));
            buffer.clear();
        }
        return tokens;
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn new<T>(tokens: &mut T) -> Option<Packet>
    where
        T: Iterator<Item = Token>,
    {
        match tokens.next().unwrap() {
            Token::OpenBracket => {
                let mut components = Vec::new();
                while let Some(p) = Packet::new(tokens) {
                    components.push(p);
                }
                Some(Packet::List(components))
            }
            Token::CloseBracket => None,
            Token::Integer(x) => Some(Packet::Integer(x)),
        }
    }
}

impl Ord for Packet {
    // cmp does a recursive lexicographic comparisons
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(x), Packet::Integer(y)) => x.cmp(y),
            (Packet::Integer(x), Packet::List(_)) => {
                Packet::List(vec![Packet::Integer(*x)]).cmp(other)
            }
            (Packet::List(_), Packet::Integer(y)) => {
                self.cmp(&Packet::List(vec![Packet::Integer(*y)]))
            }
            (Packet::List(x), Packet::List(y)) => {
                for i in 0..x.len() {
                    if i == y.len() {
                        return Ordering::Greater;
                    }
                    let elem_cmp = x[i].cmp(&y[i]);
                    if elem_cmp != Ordering::Equal {
                        return elem_cmp;
                    }
                }
                if x.len() == y.len() {
                    return Ordering::Equal;
                } else {
                    return Ordering::Less;
                }
            }
        }
    }
}

// Partial ord simply recycles the value from ord
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let file = File::open(Path::new("input")).unwrap();
    // This statement reads the lines, removes the blank ones, and parses the rest into packets
    let mut packets = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| !x.is_empty())
        .map(|x| Packet::new(&mut Token::from_str(&x).into_iter()).unwrap())
        .collect::<Vec<_>>();

    // This loop find the pairs in the right order
    let mut result = 0;
    for (i, pair) in packets.chunks(2).enumerate() {
        if pair[0] < pair[1] {
            result += i + 1;
        }
    }
    println!("Part 1: {}", result);

    let divider_packets = vec![
        Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]),
    ];

    // Here we add the divider packets and sort
    packets.push(divider_packets[0].clone());
    packets.push(divider_packets[1].clone());
    packets.sort();

    // This computes the decoder key
    let divider_1_index = packets
        .iter()
        .position(|x| x == &divider_packets[0])
        .unwrap()
        + 1;
    let divider_2_index = packets
        .iter()
        .position(|x| x == &divider_packets[1])
        .unwrap()
        + 1;
    println!("Part 2: {}", divider_1_index * divider_2_index);
}
