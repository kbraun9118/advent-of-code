use std::collections::{HashMap, VecDeque};

type ModuleConfig = HashMap<String, Module>;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Pulse {
    High,
    Low,
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct FlipFlop {
    name: String,
    state: State,
    output: Vec<String>,
}

impl FlipFlop {
    fn receive(&mut self, pulse: Pulse) -> (String, Pulse, Vec<String>) {
        match pulse {
            Pulse::High => (self.name.clone(), pulse, vec![]),
            Pulse::Low => match self.state {
                State::On => {
                    self.state = State::Off;
                    (self.name.clone(), Pulse::Low, self.output.clone())
                }
                State::Off => {
                    self.state = State::On;
                    (self.name.clone(), Pulse::High, self.output.clone())
                }
            },
        }
    }
}

impl From<&str> for FlipFlop {
    fn from(value: &str) -> Self {
        let (name, output) = value.split_once(" -> ").unwrap();
        Self {
            name: name.to_string(),
            state: State::Off,
            output: output.split(", ").map(String::from).collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    name: String,
    previous_pulse: HashMap<String, Pulse>,
    has_pulsed: HashMap<String, bool>,
    output: Vec<String>,
}

impl Conjunction {
    fn add_input(&mut self, input: String) {
        self.previous_pulse.insert(input.clone(), Pulse::Low);
        self.has_pulsed.insert(input, false);
    }

    fn receive(&mut self, from: String, pulse: Pulse) -> (String, Pulse, Vec<String>) {
        self.previous_pulse
            .entry(from.clone())
            .and_modify(|e| *e = pulse);
        if pulse == Pulse::High {
            self.has_pulsed.entry(from).and_modify(|e| *e = true);
        }

        if self.previous_pulse.values().all(|p| *p == Pulse::High) {
            (self.name.clone(), Pulse::Low, self.output.clone())
        } else {
            (self.name.clone(), Pulse::High, self.output.clone())
        }
    }
}

impl From<&str> for Conjunction {
    fn from(value: &str) -> Self {
        let (name, output) = value.split_once(" -> ").unwrap();
        Self {
            name: name.to_string(),
            output: output.split(", ").map(String::from).collect(),
            previous_pulse: HashMap::new(),
            has_pulsed: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Vec<String>),
}

impl Module {
    fn receive(&mut self, from: String, pulse: Pulse) -> (String, Pulse, Vec<String>) {
        match self {
            Module::FlipFlop(flip_flop) => flip_flop.receive(pulse),
            Module::Conjunction(conjunction) => conjunction.receive(from, pulse),
            Module::Broadcaster(output) => ("broadcaster".to_string(), pulse, output.clone()),
        }
    }

    fn unwrap_conjunction(&self) -> &Conjunction {
        if let Module::Conjunction(c) = self {
            c
        } else {
            panic!("Not conjunction")
        }
    }
}

impl From<&str> for Module {
    fn from(value: &str) -> Self {
        if value.starts_with("%") {
            Module::FlipFlop(FlipFlop::from(&value[1..]))
        } else if value.starts_with("&") {
            Module::Conjunction(Conjunction::from(&value[1..]))
        } else {
            let (_, output) = value.split_once(" -> ").unwrap();
            Module::Broadcaster(output.split(", ").map(String::from).collect())
        }
    }
}

fn parse_input(input: Vec<String>) -> ModuleConfig {
    let mut output: HashMap<_, _> = input
        .into_iter()
        .map(|s| Module::from(s.as_str()))
        .map(|m| {
            (
                match m {
                    Module::Conjunction(ref c) => c.name.clone(),
                    Module::FlipFlop(ref f) => f.name.clone(),
                    Module::Broadcaster(_) => "broadcaster".to_string(),
                },
                m,
            )
        })
        .collect();

    for (_, value) in &output.clone() {
        let (name, outputs) = match value {
            Module::FlipFlop(ref f) => (f.name.clone(), &f.output),
            Module::Conjunction(ref c) => (c.name.clone(), &c.output),
            Module::Broadcaster(ref b) => ("broadcaster".to_string(), b),
        };
        for out in outputs {
            if let Some(Module::Conjunction(c)) = output.get_mut(out) {
                c.add_input(name.clone());
            }
        }
    }

    output
}

fn press_button(module_config: &mut ModuleConfig) -> (usize, usize) {
    let mut module_outputs = VecDeque::new();
    module_outputs.push_back(
        module_config
            .get_mut(&"broadcaster".to_string())
            .unwrap()
            .receive("button".to_string(), Pulse::Low),
    );
    let mut low = 1;
    let mut high = 0;

    while let Some((from, pulse, tos)) = module_outputs.pop_front() {
        for to in tos {
            match pulse {
                Pulse::High => high += 1,
                Pulse::Low => low += 1,
            };
            if let Some(module) = module_config.get_mut(&to) {
                module_outputs.push_back(module.receive(from.clone(), pulse));
            }
        }
    }

    (low, high)
}

fn part_1(module_config: &ModuleConfig) -> usize {
    let mut module_config = module_config.clone();
    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        let (c_low, c_high) = press_button(&mut module_config);
        low += c_low;
        high += c_high;
    }

    low * high
}

fn get_rx_input_conjuction(module_config: &ModuleConfig) -> &Conjunction {
    module_config
        .values()
        .find(|m| {
            if let Module::Conjunction(conjuction) = m {
                conjuction.output.contains(&"rx".to_string())
            } else {
                false
            }
        })
        .unwrap()
        .unwrap_conjunction()
}

fn part_2(module_config: &ModuleConfig) -> usize {
    let mut module_config = module_config.clone();
    let mut i = 0;
    let rx_input_conjuction = get_rx_input_conjuction(&module_config);
    let mut rx_input_conjuction_indecies = rx_input_conjuction
        .previous_pulse
        .iter()
        .map(|(k, _)| (k.clone(), 0))
        .collect::<HashMap<_, _>>();

    loop {
        i += 1;
        let (_, _) = press_button(&mut module_config);

        let rx_input_conjuction = get_rx_input_conjuction(&module_config);

        for (key, value) in &rx_input_conjuction.has_pulsed {
            if rx_input_conjuction_indecies[key] == 0 && *value {
                rx_input_conjuction_indecies.insert(key.clone(), i);
            }
        }

        if rx_input_conjuction_indecies.values().all(|j| *j > 0) {
            return rx_input_conjuction_indecies
                .values()
                .cloned()
                .fold(1, |acc, next| num::integer::lcm(acc, next));
        }
    }
}

fn main() {
    let input = aoc::read_input_lines("20");
    aoc::benchmark(|| {
        let module_config = parse_input(input);

        aoc::print_part_1(part_1(&module_config));
        aoc::print_part_2(part_2(&module_config));
    })
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> ModuleConfig {
        parse_input(
            r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
                .lines()
                .map(String::from)
                .collect(),
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_test_input()), 11687500);
    }
}
