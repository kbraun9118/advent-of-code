use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::str::FromStr;

//this is wrong
fn main() {
    let reader = BufReader::new(
        File::open(Path::new("src/bin/day-10/input.txt")).unwrap());

    let values = reader.lines()
        .map(|line| line.unwrap())
        .nth(0)
        .map(|string| string.split(" ").collect::<Vec<&str>>()
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>())
        .map(|strings| strings.iter()
            .map(|string| usize::from_str(string).unwrap())
            .collect::<Vec<usize>>())
        .unwrap();

    let node = Node::new(&values);
    println!("{:?}", node.check_sum());
    println!("{:?}", node.check_sum_part2());
}


#[derive(Debug, Default)]
struct Node {
    children: Vec<Node>,
    meta_data: Vec<usize>,
}

impl Node {
    fn new(slice: &[usize]) -> Self {
        let (node, _) = Node::from_slice(slice);
        node
    }

    fn from_slice(slice: &[usize]) -> (Node, &[usize]) {
        let num_child = slice[0];
        let num_meta = slice[1];

        let mut slice = &slice[2..];
        let mut children = vec![];

        for _ in 0..num_child {
            let (child, rest) = Node::from_slice(slice);
            children.push(child);
            slice = rest;
        }

        let mut meta_data = vec![];

        for i in 0..num_meta {
            meta_data.push(slice[i]);
        }
        (Node { children, meta_data }, &slice[num_meta..])
    }

    fn check_sum(&self) -> usize {
        self.meta_data.iter()
            .fold(0, |left, right| left + right)
            + self.children.iter()
            .fold(0, |left, right| left + right.check_sum())
    }

    fn check_sum_part2(&self) -> usize {
        match self.children.len() {
            0 => self.meta_data.iter().sum(),
            _ => {
                let mut sum = 0;
                for meta_entry in &self.meta_data {
                    if let Some(child) = self.children.get(meta_entry - 1) {
                        sum += child.check_sum_part2()
                    }
                }
                sum
            }
        }
    }
}