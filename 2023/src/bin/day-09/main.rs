fn find_next(nums: &Vec<i32>) -> i32 {
    let mut sequences = vec![nums.clone()];
    while !sequences.last().unwrap().iter().all(|n| *n == 0) {
        let next = sequences
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();
        sequences.push(next);
    }
    let mut prev_end = 0;
    for sequence in sequences.iter().rev().skip(1) {
        prev_end = sequence.last().unwrap() + prev_end;
    }
    prev_end
}

fn find_previous(nums: &Vec<i32>) -> i32 {
    let mut sequences = vec![nums.clone()];
    while !sequences.last().unwrap().iter().all(|n| *n == 0) {
        let next = sequences
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();
        sequences.push(next);
    }
    let mut prev_start = 0;
    for sequence in sequences.iter().rev().skip(1) {
        prev_start = sequence[0] - prev_start;
    }
    prev_start
}

fn part_1(nums: &Vec<Vec<i32>>) -> i32 {
    nums.iter().map(find_next).sum()
}

fn part_2(nums: &Vec<Vec<i32>>) -> i32 {
    nums.iter().map(find_previous).sum()
}

fn main() {
    let nums = aoc::read_input_lines("09")
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    aoc::print_part_1(part_1(&nums));
    aoc::print_part_2(part_2(&nums));
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_find_next() {
        let nums = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(find_next(&nums), 18);

        let nums = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(find_next(&nums), 28);

        let nums = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(find_next(&nums), 68);
    }

    #[test]
    fn test_find_previous() {
        let nums = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(find_previous(&nums), 5);
    }
}
