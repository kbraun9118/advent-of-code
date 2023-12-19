use core::panic;
use std::collections::HashMap;

type InstructionSet = HashMap<String, Vec<Instruction>>;
type AcceptReject = aoc::Either<Toy, Toy>;

#[derive(Debug, Clone, Copy)]
struct Toy {
    extreme: usize,
    musical: usize,
    aerodynamic: usize,
    shiny: usize,
}

impl Toy {
    fn accept_or_reject(&self, instruction_set: &InstructionSet) -> AcceptReject {
        let mut current = instruction_set[&"in".to_string()].clone();

        loop {
            'current_instruction: for instruction in current.clone() {
                match instruction {
                    Instruction::Test(test) => {
                        let category = match test.category {
                            Category::Shiny => self.shiny,
                            Category::Musical => self.musical,
                            Category::Extreme => self.extreme,
                            Category::Aerodynamic => self.aerodynamic,
                        };
                        match test.condition {
                            Condition::LessThan => {
                                if category < test.value {
                                    if test.output == "A" {
                                        return AcceptReject::Left(*self);
                                    }
                                    if test.output == "R" {
                                        return AcceptReject::Right(*self);
                                    }
                                    current = instruction_set[&test.output].clone();
                                    break 'current_instruction;
                                }
                            }
                            Condition::GreaterThan => {
                                if category > test.value {
                                    if test.output == "A" {
                                        return AcceptReject::Left(*self);
                                    }
                                    if test.output == "R" {
                                        return AcceptReject::Right(*self);
                                    }
                                    current = instruction_set[&test.output].clone();
                                    break 'current_instruction;
                                }
                            }
                        }
                    }
                    Instruction::Fallback(fallback) => {
                        if fallback == "A" {
                            return AcceptReject::Left(*self);
                        }
                        if fallback == "R" {
                            return AcceptReject::Right(*self);
                        }
                        current = instruction_set[&fallback].clone();
                    }
                }
            }
        }
    }
}

impl From<&str> for Toy {
    fn from(value: &str) -> Self {
        let map = value[1..value.len() - 1]
            .split(",")
            .into_iter()
            .map(|s| s.split_once("=").unwrap())
            .collect::<HashMap<_, _>>();
        Self {
            extreme: map["x"].parse().unwrap(),
            musical: map["m"].parse().unwrap(),
            aerodynamic: map["a"].parse().unwrap(),
            shiny: map["s"].parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Category {
    Extreme,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Clone, Copy)]
enum Condition {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
struct Test {
    category: Category,
    condition: Condition,
    value: usize,
    output: String,
}

impl From<&str> for Test {
    fn from(value: &str) -> Self {
        let category = match &value[0..1] {
            "x" => Category::Extreme,
            "m" => Category::Musical,
            "a" => Category::Aerodynamic,
            "s" => Category::Shiny,
            _ => panic!("invalid char"),
        };
        let condition = match &value[1..2] {
            ">" => Condition::GreaterThan,
            "<" => Condition::LessThan,
            _ => panic!("invalid char"),
        };
        let (value, output) = value[2..].split_once(":").unwrap();
        let value = value.parse().unwrap();
        let output = output.to_string();

        Self {
            category,
            condition,
            value,
            output,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Test(Test),
    Fallback(String),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if value.contains(":") {
            Instruction::Test(Test::from(value))
        } else {
            Instruction::Fallback(value.to_string())
        }
    }
}

fn part_1(instruction_set: &InstructionSet, toys: &Vec<Toy>) -> usize {
    let mut accepted = vec![];

    for toy in toys {
        if let AcceptReject::Left(accept) = toy.accept_or_reject(instruction_set) {
            accepted.push(accept);
        }
    }

    accepted
        .into_iter()
        .map(|t| t.aerodynamic + t.musical + t.extreme + t.shiny)
        .sum()
}

fn parse_input(input: Vec<String>) -> (InstructionSet, Vec<Toy>) {
    let (instructions, toys) = input.split_at(input.iter().position(|l| l == "").unwrap());
    let toys = toys[1..].iter().map(|t| Toy::from(t.as_str())).collect();

    let instructions = instructions
        .iter()
        .map(|l| {
            let (key, value) = l.split_once("{").unwrap();
            let value = value[0..value.len() - 1]
                .split(",")
                .map(|v| Instruction::from(v))
                .collect();
            (key.to_string(), value)
        })
        .collect();

    (instructions, toys)
}

fn main() {
    let (instructions, toys) = parse_input(aoc::read_input_lines("19"));

    aoc::print_part_1(part_1(&instructions, &toys));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> (InstructionSet, Vec<Toy>) {
        let input = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
            .lines()
            .map(String::from)
            .collect();
        parse_input(input)
    }

    #[test]
    fn test_part_1() {
        let (instructions, toys) = get_test_input();
        assert_eq!(part_1(&instructions, &toys), 19114);
    }
}

