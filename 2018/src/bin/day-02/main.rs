use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::collections::HashMap;
use std::ops::Deref;

fn main() {
    let strings: Vec<String> = BufReader::new(
        File::open(Path::new("../input/2018/02/input.txt")).unwrap())
        .lines()
        .map(|string| string.unwrap())
        .collect();


    let (twos, threes) = strings.iter()
        .map(|line| {
            let mut letters = Letters::new();
            line.chars().for_each(|chars| letters.push(chars));
            letters
        }).map(|letters| letters.test())
        .fold((0, 0), |(twos, threes), (two, three)| (twos + two as u32, threes + three as u32));


    let slice = strings;

    let mut answer = (None, None);

    let mut vec = Vec::new();

    'a: for i in 0..slice.len() {
        let i_chars = &slice[i];
        for j in i + 1..slice.len() {
            let j_chars = &slice[j];
            for index in 0..i_chars.len() {
                vec.push(i_chars.as_bytes()[index] as char == j_chars.as_bytes()[index] as char);
            }
            if vec.iter().filter(|&&bools| bools == false).count() == 1 {
                answer = (Some(i), Some(j));
                break 'a;
            }
            vec.clear();
        }
    };
    println!("Part 1: {}", twos * threes);
    if let (Some(first), Some(second)) = answer {
        println!("Part 2: \n{}, \n{}", slice[first], slice[second]);
    }
}

struct Letters(HashMap<char, u8>);

impl Deref for Letters {
    type Target = HashMap<char, u8>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl Letters {
    fn new() -> Letters {
        Letters(HashMap::new())
    }

    fn push(&mut self, item: char) {
        if let Some(value) = self.get(&item) {
            self.0.insert(item, value + 1);
        } else {
            self.0.insert(item, 1);
        }
    }

    fn test(&self) -> (bool, bool) {
        (self.values().any(|num| *num == 2),
         self.values().any(|num| *num == 3))
    }
}