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
    state: State,
    output: Vec<String>,
}

impl FlipFlop {
    fn receive(&mut self, pulse: Pulse) -> (Pulse, Vec<String>) {
        match pulse {
            Pulse::High => (pulse, vec![]),
            Pulse::Low => match self.state {
                State::On => {
                    self.state = State::Off;
                    (Pulse::Low, self.output.clone())
                }
                State::Off => {
                    self.state = State::On;
                    (Pulse::High, self.output.clone())
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    previous_pulse: Pulse,
    output: Vec<String>,
}

impl Conjunction {
    fn receive(&mut self, pulse: Pulse) -> (Pulse, Vec<String>) {
        self.previous_pulse = pulse; 
    }
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Vec<String>),
}
fn main() {
    println!("Hello");
}
