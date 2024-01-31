use std::cell::RefCell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::rc::Rc;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::Range;

fn main() {
    let reader = BufReader::new(File::open(Path::new("../input/2018/12/input.txt")).unwrap());

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let split: Vec<&str> = lines[0].split(" ")
        .collect();

    let mut decision_tree = DecisionTree::new();

    let sequence = into_sequence(&split[2].chars().collect::<Vec<char>>());

    let mut pots = Pots::from_sequence(&sequence);

    let patterns = &lines[2..lines.len()];

    for pattern in patterns {
        let split: Vec<&str> = pattern.split(" ")
            .collect();

        let collected = String::from(split[0]) + split[2];
        let sequence = into_sequence(&collected.chars().collect::<Vec<char>>());
        decision_tree.add_sequence(&sequence);
    }

    print_sequence(&pots);

    for i in 0..2000 {
        println!("Generation: {}", i + 1);
        pots.process_generation(&decision_tree);
        print_sequence(&pots);
        println!("{:?}", pots.count_score());
    }
}

fn print_sequence(sequence: &[Status]) {
    for status in sequence {
        print!("{}", match status {
            Status::Plant => '#',
            Status::Empty => '.'
        })
    }
    println!()
}

#[derive(Debug)]
struct Pots(Vec<Status>);

impl Pots {
    fn from_sequence(sequence: &[Status]) -> Self {
        let mut pots = vec![];

        for _ in 0..2000 {
            pots.push(Status::Empty)
        }

        for status in sequence {
            pots.push(*status)
        }

        for _ in 0..10000 {
            pots.push(Status::Empty)
        }

        Self(pots)
    }

    fn process_generation(&mut self, decision_tree: &DecisionTree) {
        let mut new = vec![];
        new.push(Status::Empty);
        new.push(Status::Empty);

        for i in 2..self.0.len() - 2 {
//            println!("Testing: position: {}, status: {:?}, with pattern:",
//                     i, self.0[i]);
//            print_sequence(&self.0[i - 2..=i + 2]);
//            println!("Answer: {:?}", decision_tree.get_answer(&self.0[i - 2..=i + 2]));

            new.push(decision_tree.get_answer(&self.0[i - 2..=i + 2]));
        }

        new.push(Status::Empty);
        new.push(Status::Empty);
        self.0 = new;
    }

    fn count_score(&self) -> isize {
        let mut score = 0;
        for i in -2000..((self.len() - 2000) as isize) {
            if let Status::Plant = self[i] {
                score += i;
            }
        }
        score
    }
}

impl Index<isize> for Pots {
    type Output = Status;

    fn index(&self, index: isize) -> &<Self as Index<isize>>::Output {
        &self.0[(index + 2000) as usize]
    }
}

impl Index<Range<isize>> for Pots {
    type Output = [Status];

    fn index(&self, index: Range<isize>) -> &<Self as Index<Range<isize>>>::Output {
        &self.0[((index.start + 2000) as usize)..(index.end as usize)]
    }
}

impl Deref for Pots {
    type Target = Vec<Status>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Pots {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

fn into_sequence(chars: &[char]) -> Vec<Status> {
    let mut vec = vec![];
    for char in chars {
        match char {
            '#' => vec.push(Status::Plant),
            '.' => vec.push(Status::Empty),
            _ => panic!("Unexpected character")
        }
    }
    vec
}

#[derive(Debug)]
struct DecisionTree {
    base_node: Rc<RefCell<TreeNode>>,
}

impl DecisionTree {
    fn new() -> Self {
        DecisionTree {
            base_node: Rc::new(RefCell::new(TreeNode::Decision(Node::new()))),
        }
    }

    fn add_sequence(&mut self, statuses: &[Status]) {
        assert_eq!(statuses.len(), 6);

        let mut current_node = self.base_node.clone();

        for i in 0..4 {
            current_node = current_node
                .clone()
                .borrow_mut()
                .give_next_decision(statuses[i])
        }

        current_node.clone().borrow_mut().set_answer(statuses[4], statuses[5]);
    }

    fn get_answer(&self, statuses: &[Status]) -> Status {
        assert_eq!(statuses.len(), 5);

        let mut current = self.base_node.clone();

        for status in &statuses[0..4] {
            match *current.clone().borrow() {
                TreeNode::Decision(ref node) => match status {
                    Status::Plant => match &node.next_plant {
                        Some(decision) => current = decision.clone(),
                        None => return Status::Empty
                    },
                    Status::Empty => match &node.next_empty {
                        Some(decision) => current = decision.clone(),
                        None => return Status::Empty
                    }
                },
                _ => panic!("Answer not allowed here.")
            }
        }

        match *current.clone().borrow() {
            TreeNode::Decision(ref node) => match statuses[4] {
                Status::Plant => match &node.next_plant {
                    Some(answer) => match *answer.borrow() {
                        TreeNode::Answer(status) => status,
                        _ => panic!("Decision not allowed here")
                    },
                    None => Status::Empty
                },
                Status::Empty => match &node.next_empty {
                    Some(answer) => match *answer.borrow() {
                        TreeNode::Answer(status) => status,
                        _ => panic!("Decision not allowed here")
                    },
                    None => Status::Empty
                }
            },
            _ => panic!("Answer not allowed here.")
        }
    }
}

#[derive(Debug)]
struct Node {
    next_empty: Option<Rc<RefCell<TreeNode>>>,
    next_plant: Option<Rc<RefCell<TreeNode>>>,
}

impl Node {
    fn new() -> Self {
        Node {
            next_empty: None,
            next_plant: None,
        }
    }
}

#[derive(Debug)]
enum TreeNode {
    Decision(Node),
    Answer(Status),
}

impl TreeNode {
    fn give_next_decision(&mut self, status: Status) -> Rc<RefCell<TreeNode>> {
        match self {
            TreeNode::Decision(decision) => match status {
                Status::Plant => match decision.next_plant.clone() {
                    Some(rc) => rc.clone(),
                    None => {
                        let node = Rc::new(RefCell::new(TreeNode::Decision(Node::new())));
                        decision.next_plant = Some(node.clone());
                        node
                    }
                },
                Status::Empty => match decision.next_empty.clone() {
                    Some(rc) => rc.clone(),
                    None => {
                        let node = Rc::new(RefCell::new(TreeNode::Decision(Node::new())));
                        decision.next_empty = Some(node.clone());
                        node
                    }
                },
            },
            _ => panic!("Cannot get decision"),
        }
    }

    fn set_answer(&mut self, status: Status, answer: Status) {
        match self {
            TreeNode::Decision(decision) => match status {
                Status::Plant => {
                    decision.next_plant = Some(Rc::new(RefCell::new(TreeNode::Answer(answer))))
                }
                Status::Empty => {
                    decision.next_empty = Some(Rc::new(RefCell::new(TreeNode::Answer(answer))))
                }
            },
            _ => panic!("Cannot be an answer"),
        }
    }
}

impl From<Node> for TreeNode {
    fn from(node: Node) -> Self {
        TreeNode::Decision(node)
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
enum Status {
    Empty,
    Plant,
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn this_works() {
        assert_eq!(1, 1)
    }
}
