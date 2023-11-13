#![feature(hash_drain_filter)]

use aoc_2015::lines_for_day;
use std::collections::HashMap;

fn main() {
    let mut instructions = lines_for_day("day-07")
        .into_iter()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").expect("Could not split");
            (right.to_string(), Connection::from(left))
        })
        .collect::<HashMap<String, Connection>>();

    let part_one_answer = solve_circuits(instructions.clone());
    println!("Part One: {}", part_one_answer);
    instructions.insert(
        String::from("b"),
        Connection::Value(format!("{}", part_one_answer)),
    );
    println!("Part Two: {}", solve_circuits(instructions));
}

fn solve_circuits(mut instructions: HashMap<String, Connection>) -> u16 {
    let mut circuits = instructions
        .drain_filter(|_, conn| {
            if let Connection::Value(val) = conn {
                val.parse::<u16>().is_ok()
            } else {
                false
            }
        })
        .map(|(s, conn)| {
            (
                s,
                if let Connection::Value(val) = conn {
                    val.parse().expect("could not parse")
                } else {
                    0
                },
            )
        })
        .collect::<HashMap<String, u16>>();

    while !instructions.clone().is_empty() {
        for (key, value) in instructions.clone().iter() {
            if let Some(value) = value.solve(&circuits) {
                circuits.insert(key.clone(), value);
                instructions.remove(key);
            }
        }
    }
    circuits[&String::from("a")]
}

#[derive(Debug, Clone)]
enum Connection {
    Value(String),
    And(String, String),
    Or(String, String),
    LShift(String, u16),
    RShift(String, u16),
    Not(String),
}

impl Connection {
    fn solve(&self, circuits: &HashMap<String, u16>) -> Option<u16> {
        use Connection::*;
        match self {
            Value(val) => {
                if circuits.contains_key(val) {
                    Some(circuits[val])
                } else {
                    None
                }
            }
            And(left, right) => {
                if circuits.contains_key(left) && circuits.contains_key(right) {
                    Some(circuits[left] & circuits[right])
                } else if let (Ok(left), true) = (left.parse::<u16>(), circuits.contains_key(right))
                {
                    Some(left & circuits[right])
                } else if let (true, Ok(right)) =
                    (circuits.contains_key(left), right.parse::<u16>())
                {
                    Some(circuits[left] & right)
                } else {
                    None
                }
            }
            Or(left, right) => {
                if circuits.contains_key(left) && circuits.contains_key(right) {
                    Some(circuits[left] | circuits[right])
                } else if let (Ok(left), true) = (left.parse::<u16>(), circuits.contains_key(right))
                {
                    Some(left | circuits[right])
                } else if let (true, Ok(right)) =
                    (circuits.contains_key(left), right.parse::<u16>())
                {
                    Some(circuits[left] | right)
                } else {
                    None
                }
            }
            LShift(left, right) => {
                if circuits.contains_key(left) {
                    Some(circuits[left] << right)
                } else {
                    None
                }
            }
            RShift(left, right) => {
                if circuits.contains_key(left) {
                    Some(circuits[left] >> right)
                } else {
                    None
                }
            }
            Not(value) => {
                if circuits.contains_key(value) {
                    Some(!circuits[value])
                } else {
                    None
                }
            }
        }
    }
}

impl From<&str> for Connection {
    fn from(string: &str) -> Self {
        use Connection::*;
        if string.starts_with("NOT") {
            Not(string[4..].to_string())
        } else if string.contains("AND") {
            let (left, right) = string.split_once(" AND ").expect("Could not split");
            And(left.to_string(), right.to_string())
        } else if string.contains("OR") {
            let (left, right) = string.split_once(" OR ").expect("Could not split");
            Or(left.to_string(), right.to_string())
        } else if string.contains("LSHIFT") {
            let (left, right) = string.split_once(" LSHIFT ").expect("Could not split");
            LShift(left.to_string(), right.parse().expect("Could not parse"))
        } else if string.contains("RSHIFT") {
            let (left, right) = string.split_once(" RSHIFT ").expect("Could not split");
            RShift(left.to_string(), right.parse().expect("Could not parse"))
        } else {
            Value(
                string
                    .parse()
                    .expect(&*format!("Could not parse: {}", string)),
            )
        }
    }
}
