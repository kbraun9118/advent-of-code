fn main() {
    let input = 34_000_000;

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: usize) -> usize {
    let mut houses: Vec<usize> = vec![0; input / 10];
    for i in 1..houses.len() {
        for j in (i..houses.len()).step_by(i) {
            houses[j] += i * 10;
        }
    }

    for (i, presents) in houses.into_iter().enumerate() {
        if presents >= input {
            return i;
        }
    }

    0
}

fn part_2(input: usize) -> usize {
    let mut houses: Vec<usize> = vec![0; input / 10];
    for i in 1..houses.len() {
        for j in (i..houses.len()).step_by(i).take(50) {
            houses[j] += i * 11;
        }
    }

    for (i, presents) in houses.into_iter().enumerate() {
        if presents >= input {
            return i;
        }
    }

    0
}
