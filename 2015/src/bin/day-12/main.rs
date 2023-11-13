use std::{collections::HashMap, vec};

use aoc_2015::lines_for_day;

fn main() {
    let input = lines_for_day("day-12").remove(0);
    let json = Json::parse(&input);
    println!("Part One: {}", json.part_one());
    println!("Part Two: {}", json.part_two());
}

enum Json {
    String(String),
    Num(i32),
    Boolean(bool),
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    Null,
}

impl Json {
    fn parse(input: &str) -> Self {
        Self::parse_iter(input).0
    }

    fn parse_iter(input: &str) -> (Self, String) {
        if input.starts_with("null") {
            (
                Json::Null,
                input
                    .strip_prefix("null")
                    .expect("Does not start with null")
                    .to_string(),
            )
        } else if input.starts_with('{') {
            let mut output = input
                .strip_prefix('{')
                .expect("Does not start with {")
                .to_string();
            let mut map = HashMap::new();
            while !output.starts_with('}') {
                if output.starts_with(',') {
                    output.remove(0);
                }
                output = output
                    .strip_prefix('"')
                    .expect("Does not start with \"")
                    .to_string();
                let (key, rest) = output.split_once("\":").expect("No closing \"");
                let (value, rest) = Self::parse_iter(rest);
                map.insert(key.to_string(), value);
                output = rest.to_string();
            }
            output.remove(0);
            (Json::Object(map), output)
        } else if input.starts_with('[') {
            let mut output = input
                .strip_prefix('[')
                .expect("Does not start with [")
                .to_string();
            let mut arr = vec![];
            while !output.starts_with(']') {
                if output.starts_with(',') {
                    output.remove(0);
                }
                let (json, rest) = Self::parse_iter(&output);
                arr.push(json);
                output = rest;
            }
            output.remove(0);
            (Json::Array(arr), output)
        } else if input.starts_with('"') {
            let output = input
                .strip_prefix('"')
                .expect("Does not start with \"")
                .to_string();
            let (string, output) = output.split_once('"').expect("No Closing \"");
            (Json::String(string.to_string()), output.to_string())
        } else if input.starts_with("true") {
            let output = input.strip_prefix("true").expect("Is not true").to_string();
            (Json::Boolean(true), output)
        } else if input.starts_with("false") {
            let output = input
                .strip_prefix("false")
                .expect("Is not true")
                .to_string();
            (Json::Boolean(true), output)
        } else {
            let split = input
                .char_indices()
                .find(|(_, c)| !(c.is_numeric() || *c == '-'))
                .expect("Could not split")
                .0;
            let (num, rest) = input.split_at(split);
            (
                Json::Num(num.parse::<i32>().expect("Could not parse")),
                rest.to_string(),
            )
        }
    }

    fn part_one(&self) -> i32 {
        match self {
            Json::String(_) => 0,
            Json::Num(value) => *value,
            Json::Array(arr) => arr.iter().map(|j| j.part_one()).sum(),
            Json::Boolean(_) => 0,
            Json::Object(obj) => obj.values().map(|j| j.part_one()).sum(),
            Json::Null => 0,
        }
    }

    fn part_two(&self) -> i32 {
        match self {
            Json::String(_) => 0,
            Json::Num(value) => *value,
            Json::Array(arr) => arr.iter().map(|j| j.part_two()).sum(),
            Json::Boolean(_) => 0,
            Json::Object(obj) => {
                if obj.values().any(|v| {
                    if let Json::String(string) = v {
                        string == "red"
                    } else {
                        false
                    }
                }) {
                    0
                } else {
                    obj.values().map(|j| j.part_two()).sum()
                }
            }
            Json::Null => 0,
        }
    }
}
