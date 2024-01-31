use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let input = File::open(Path::new("../input/2018/01/input.txt")).unwrap();
    let reader = BufReader::new(input);

    let mut map = HashMap::new();
    map.insert(0, 0);
    let mut answer2: Option<i32> = None;

    let collected = reader.lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut total = 0;

    while answer2.is_none() {
        for num in &collected {
            total += num;
            if answer2.is_none() {
                answer2 = map.insert(total, total)
            };
        }
    }
    println!("Answer: {:?}", total);
    println!("Answer3: {:?}", answer2.unwrap());
}
