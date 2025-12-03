use aoc_2025::{Error, print_output, read_input};

fn find_joltage(nums: &Vec<u64>, window_size: usize) -> u64 {
    let mut digits = vec![0u64; window_size];

    for nums in nums.windows(window_size) {
        for (i, &num) in nums.iter().enumerate() {
            if num > digits[i] {
                digits[i..].copy_from_slice(&nums[i..]);
            }
        }
    }

    digits.into_iter().enumerate().fold(0, |acc, (i, num)| {
        acc + num * 10u64.pow((window_size as u32 - 1) - (i as u32))
    })
}

fn main() -> Result<(), Error> {
    let input = read_input("03")?;

    let banks = input
        .into_iter()
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|n| n as u64)
                        .ok_or(format!("invalid input character: {}", c))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let part_1 = banks.iter().map(|nums| find_joltage(nums, 2)).sum::<u64>();
    let part_2 = banks.iter().map(|nums| find_joltage(nums, 12)).sum::<u64>();

    print_output!(part_1, part_2);
    Ok(())
}
